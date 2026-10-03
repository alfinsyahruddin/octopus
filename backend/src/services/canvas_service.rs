use std::{collections::HashMap, sync::Arc, time::Instant};

use crate::{
    entities::{
        app_error::AppError,
        decision::{DecisionModelInfo, EvaluateDecisionRequest, EvaluateDecisionResponse},
    },
    traits::decision_engine::DecisionEngine,
};

pub struct CanvasService {
    engines: HashMap<String, Arc<dyn DecisionEngine>>,
    default_model: String,
}

impl CanvasService {
    pub fn new(engines: Vec<Arc<dyn DecisionEngine>>, default_model: impl Into<String>) -> Self {
        let mut map = HashMap::new();
        for engine in engines {
            map.insert(engine.model_id().to_string(), engine);
        }
        Self {
            engines: map,
            default_model: default_model.into(),
        }
    }

    pub fn list_models(&self) -> Vec<DecisionModelInfo> {
        self.engines.values().map(|e| e.model_info()).collect()
    }

    pub async fn evaluate(
        &self,
        req: &EvaluateDecisionRequest,
    ) -> Result<EvaluateDecisionResponse, AppError> {
        if req.instruction.trim().is_empty() {
            return Err(AppError::bad_request("Instruction cannot be empty"));
        }
        if req.prompt.trim().is_empty() {
            return Err(AppError::bad_request("Prompt cannot be empty"));
        }

        let model_id = if req.model.trim().is_empty() {
            &self.default_model
        } else {
            &req.model
        };

        let engine = self.engines.get(model_id).ok_or_else(|| {
            AppError::not_found(format!("Model '{model_id}' is not supported or registered"))
        })?;

        let start_time = Instant::now();
        let result = engine.evaluate(req).await?;
        let ai_duration_ms = start_time.elapsed().as_secs_f64() * 1000.0;

        Ok(EvaluateDecisionResponse {
            model: model_id.clone(),
            model_type: req.model_type,
            result,
            ai_duration_ms,
        })
    }
}
