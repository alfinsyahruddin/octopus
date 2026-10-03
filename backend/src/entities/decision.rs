use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DecisionModelType {
    Choice,
    Score,
    Noul,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub supported_types: Vec<DecisionModelType>,
    pub supports_vision: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceItem {
    pub id: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoulCriteria {
    pub true_desc: Option<String>,
    pub false_desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateDecisionRequest {
    pub model: String,
    pub model_type: DecisionModelType,
    pub instruction: String,
    pub prompt: String,
    #[serde(default)]
    pub images: Vec<String>,
    pub choices: Option<Vec<ChoiceItem>>,
    pub score_levels: Option<Vec<String>>,
    pub noul_criteria: Option<NoulCriteria>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceResult {
    pub choice: String,
    pub probabilities: BTreeMap<String, f64>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreResult {
    pub score: f64,
    pub legend: BTreeMap<String, String>,
    pub probabilities: BTreeMap<String, f64>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoulResult {
    pub noul: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DecisionResultUnion {
    Choice(ChoiceResult),
    Score(ScoreResult),
    Noul(NoulResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateDecisionResponse {
    pub model: String,
    pub model_type: DecisionModelType,
    pub result: DecisionResultUnion,
    pub ai_duration_ms: f64,
}
