use async_trait::async_trait;

use crate::entities::{
    app_error::AppError,
    decision::{DecisionModelInfo, DecisionResultUnion, EvaluateDecisionRequest},
};

#[async_trait]
pub trait DecisionEngine: Send + Sync {
    fn model_id(&self) -> &str;
    fn model_info(&self) -> DecisionModelInfo;
    async fn evaluate(
        &self,
        req: &EvaluateDecisionRequest,
    ) -> Result<DecisionResultUnion, AppError>;
}
