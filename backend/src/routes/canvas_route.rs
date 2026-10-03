use actix_web::{get, post, web, Scope};

use crate::{
    entities::{
        app_response::{AppResponse, IntoResponseTrait},
        decision::{DecisionModelInfo, EvaluateDecisionRequest, EvaluateDecisionResponse},
    },
    services::canvas_service::CanvasService,
};

pub fn canvas_scope() -> Scope {
    web::scope("/canvas")
        .service(get_models)
        .service(evaluate_decision)
}

#[get("/models")]
pub async fn get_models(
    canvas_service: web::Data<CanvasService>,
) -> AppResponse<Vec<DecisionModelInfo>> {
    let models = canvas_service.list_models();
    Ok(models).json()
}

#[post("/evaluate")]
pub async fn evaluate_decision(
    canvas_service: web::Data<CanvasService>,
    payload: web::Json<EvaluateDecisionRequest>,
) -> AppResponse<EvaluateDecisionResponse> {
    let res = canvas_service.evaluate(&payload.into_inner()).await?;
    Ok(res).json()
}
