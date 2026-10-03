use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::collections::BTreeMap;

use crate::{
    entities::{
        app_error::AppError,
        decision::{
            ChoiceResult, DecisionModelInfo, DecisionModelType, DecisionResultUnion,
            EvaluateDecisionRequest, NoulResult, ScoreResult,
        },
    },
    traits::decision_engine::DecisionEngine,
};

pub struct OllamaEngine {
    client: Client,
    base_url: String,
    model_id: String,
}

impl OllamaEngine {
    pub fn new(client: Client, base_url: String, model_id: impl Into<String>) -> Self {
        Self {
            client,
            base_url,
            model_id: model_id.into(),
        }
    }

    fn clean_base64_image(img: &str) -> String {
        if let Some(pos) = img.find(";base64,") {
            img[(pos + 8)..].to_string()
        } else {
            img.to_string()
        }
    }
}

#[async_trait]
impl DecisionEngine for OllamaEngine {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn model_info(&self) -> DecisionModelInfo {
        DecisionModelInfo {
            id: self.model_id.clone(),
            name: "Clef Flash 9B".to_string(),
            description: "Fast 9B multimodal System One decision model by Cloudflare".to_string(),
            supported_types: vec![
                DecisionModelType::Choice,
                DecisionModelType::Score,
                DecisionModelType::Noul,
            ],
            supports_vision: true,
        }
    }

    async fn evaluate(
        &self,
        req: &EvaluateDecisionRequest,
    ) -> Result<DecisionResultUnion, AppError> {
        let question_payload = match req.model_type {
            DecisionModelType::Choice => {
                let choices = req.choices.as_ref().ok_or_else(|| {
                    AppError::bad_request("Choices list is required for Choice decision")
                })?;
                if choices.len() < 2 {
                    return Err(AppError::bad_request("At least 2 choices are required"));
                }

                let mut criteria_map = serde_json::Map::new();
                for c in choices {
                    let desc = c.description.as_deref().unwrap_or(c.id.as_str());
                    criteria_map.insert(c.id.clone(), json!(desc));
                }

                json!({
                    "type": "choice",
                    "instructions": req.instruction,
                    "criteria": criteria_map
                })
            }
            DecisionModelType::Score => {
                let levels = req.score_levels.as_ref().ok_or_else(|| {
                    AppError::bad_request("Score levels are required for Score decision")
                })?;
                if levels.len() < 2 {
                    return Err(AppError::bad_request(
                        "At least 2 score levels are required",
                    ));
                }

                json!({
                    "type": "score",
                    "instructions": req.instruction,
                    "criteria": levels
                })
            }
            DecisionModelType::Noul => {
                let mut noul_obj = json!({
                    "type": "noul",
                    "instructions": req.instruction
                });

                if let Some(criteria) = &req.noul_criteria {
                    let mut c_map = serde_json::Map::new();
                    if let Some(t) = &criteria.true_desc {
                        if !t.trim().is_empty() {
                            c_map.insert("true".to_string(), json!(t));
                        }
                    }
                    if let Some(f) = &criteria.false_desc {
                        if !f.trim().is_empty() {
                            c_map.insert("false".to_string(), json!(f));
                        }
                    }
                    if !c_map.is_empty() {
                        noul_obj["criteria"] = json!(c_map);
                    }
                }

                noul_obj
            }
        };

        let mut body = json!({
            "model": self.model_id,
            "state": req.prompt,
            "questions": {
                "decision": question_payload
            }
        });

        if !req.images.is_empty() {
            let cleaned_images: Vec<String> = req
                .images
                .iter()
                .map(|img| Self::clean_base64_image(img))
                .collect();
            body["images"] = json!(cleaned_images);
        }

        let endpoint = format!("{}/v1/systemone", self.base_url.trim_end_matches('/'));
        let response = self
            .client
            .post(&endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                eprintln!("Ollama request error: {e}");
                AppError::internal(format!("Failed to connect to inference engine: {e}"))
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::internal(format!(
                "Inference engine returned error {status}: {error_text}"
            )));
        }

        let resp_json: serde_json::Value = response.json().await.map_err(|e| {
            AppError::internal(format!("Failed to parse JSON response from engine: {e}"))
        })?;

        let answer = resp_json
            .get("answers")
            .and_then(|a| a.get("decision"))
            .ok_or_else(|| {
                AppError::internal("Missing 'answers.decision' in inference response".to_string())
            })?;

        match req.model_type {
            DecisionModelType::Choice => {
                let choice = answer
                    .get("choice")
                    .and_then(|c| c.as_str())
                    .unwrap_or("")
                    .to_string();

                let mut probabilities = BTreeMap::new();
                if let Some(probs) = answer.get("probabilities").and_then(|p| p.as_object()) {
                    for (k, v) in probs {
                        probabilities.insert(k.clone(), v.as_f64().unwrap_or(0.0));
                    }
                }

                let confidence = answer
                    .get("confidence")
                    .and_then(|c| c.as_f64())
                    .unwrap_or(0.0);

                Ok(DecisionResultUnion::Choice(ChoiceResult {
                    choice,
                    probabilities,
                    confidence,
                }))
            }
            DecisionModelType::Score => {
                let score = answer.get("score").and_then(|s| s.as_f64()).unwrap_or(0.0);

                let mut legend = BTreeMap::new();
                if let Some(leg) = answer.get("legend").and_then(|l| l.as_object()) {
                    for (k, v) in leg {
                        legend.insert(k.clone(), v.as_str().unwrap_or("").to_string());
                    }
                }

                let mut probabilities = BTreeMap::new();
                if let Some(probs) = answer.get("probabilities").and_then(|p| p.as_object()) {
                    for (k, v) in probs {
                        probabilities.insert(k.clone(), v.as_f64().unwrap_or(0.0));
                    }
                }

                let confidence = answer
                    .get("confidence")
                    .and_then(|c| c.as_f64())
                    .unwrap_or(0.0);

                Ok(DecisionResultUnion::Score(ScoreResult {
                    score,
                    legend,
                    probabilities,
                    confidence,
                }))
            }
            DecisionModelType::Noul => {
                let noul = answer.get("noul").and_then(|n| n.as_f64()).unwrap_or(0.5);

                Ok(DecisionResultUnion::Noul(NoulResult { noul }))
            }
        }
    }
}
