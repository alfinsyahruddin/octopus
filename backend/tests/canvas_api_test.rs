use actix_web::{test, App};
use async_trait::async_trait;
use std::collections::BTreeMap;
use std::sync::Arc;

use backend::{
    entities::{
        app_error::AppError,
        base_response::BaseResponse,
        decision::{
            ChoiceItem, ChoiceResult, DecisionModelInfo, DecisionModelType, DecisionResultUnion,
            EvaluateDecisionRequest, EvaluateDecisionResponse, NoulResult, ScoreResult,
        },
    },
    http, routes,
    services::canvas_service::CanvasService,
    traits::decision_engine::DecisionEngine,
};

struct MockEngine;

#[async_trait]
impl DecisionEngine for MockEngine {
    fn model_id(&self) -> &str {
        "mock-model"
    }

    fn model_info(&self) -> DecisionModelInfo {
        DecisionModelInfo {
            id: "mock-model".to_string(),
            name: "Mock Decision Engine".to_string(),
            description: "Test mock engine".to_string(),
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
        match req.model_type {
            DecisionModelType::Choice => {
                let mut probs = BTreeMap::new();
                probs.insert("opt1".to_string(), 0.9);
                probs.insert("opt2".to_string(), 0.1);
                Ok(DecisionResultUnion::Choice(ChoiceResult {
                    choice: "opt1".to_string(),
                    probabilities: probs,
                    confidence: 0.85,
                }))
            }
            DecisionModelType::Score => {
                let mut probs = BTreeMap::new();
                probs.insert("0".to_string(), 0.2);
                probs.insert("1".to_string(), 0.8);
                let mut legend = BTreeMap::new();
                legend.insert("0".to_string(), "Low".to_string());
                legend.insert("1".to_string(), "High".to_string());
                Ok(DecisionResultUnion::Score(ScoreResult {
                    score: 0.8,
                    legend,
                    probabilities: probs,
                    confidence: 0.75,
                }))
            }
            DecisionModelType::Noul => Ok(DecisionResultUnion::Noul(NoulResult { noul: 0.95 })),
        }
    }
}

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().configure(routes::configure)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body: BaseResponse<String> = test::read_body_json(resp).await;
    assert_eq!(body.data, Some("ok".to_string()));
    assert_eq!(body.status, 200);
}

#[actix_web::test]
async fn test_canvas_models_endpoint() {
    let mock_engine: Arc<dyn DecisionEngine> = Arc::new(MockEngine);
    let service = CanvasService::new(vec![mock_engine], "mock-model");
    let canvas_data = actix_web::web::Data::new(service);

    let app = test::init_service(
        App::new()
            .app_data(canvas_data)
            .configure(routes::configure),
    )
    .await;

    let req = test::TestRequest::get().uri("/canvas/models").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body: BaseResponse<Vec<DecisionModelInfo>> = test::read_body_json(resp).await;
    let models = body.data.expect("models data must be present");
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id, "mock-model");
}

#[actix_web::test]
async fn test_canvas_evaluate_choice() {
    let mock_engine: Arc<dyn DecisionEngine> = Arc::new(MockEngine);
    let service = CanvasService::new(vec![mock_engine], "mock-model");
    let canvas_data = actix_web::web::Data::new(service);

    let app = test::init_service(
        App::new()
            .app_data(http::json_config())
            .app_data(canvas_data)
            .configure(routes::configure),
    )
    .await;

    let payload = EvaluateDecisionRequest {
        model: "mock-model".to_string(),
        model_type: DecisionModelType::Choice,
        instruction: "Select the best option".to_string(),
        prompt: "Here is a test inquiry".to_string(),
        images: vec![],
        choices: Some(vec![
            ChoiceItem {
                id: "opt1".to_string(),
                description: Some("Option 1".to_string()),
            },
            ChoiceItem {
                id: "opt2".to_string(),
                description: Some("Option 2".to_string()),
            },
        ]),
        score_levels: None,
        noul_criteria: None,
    };

    let req = test::TestRequest::post()
        .uri("/canvas/evaluate")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body: BaseResponse<EvaluateDecisionResponse> = test::read_body_json(resp).await;
    let eval_res = body.data.expect("evaluation response must be present");
    assert_eq!(eval_res.model, "mock-model");
    assert_eq!(eval_res.model_type, DecisionModelType::Choice);
    assert!(eval_res.ai_duration_ms >= 0.0);

    match eval_res.result {
        DecisionResultUnion::Choice(c) => {
            assert_eq!(c.choice, "opt1");
            assert_eq!(c.confidence, 0.85);
        }
        _ => panic!("Expected Choice result"),
    }
}

#[actix_web::test]
async fn test_canvas_evaluate_validation_error() {
    let mock_engine: Arc<dyn DecisionEngine> = Arc::new(MockEngine);
    let service = CanvasService::new(vec![mock_engine], "mock-model");
    let canvas_data = actix_web::web::Data::new(service);

    let app = test::init_service(
        App::new()
            .app_data(http::json_config())
            .app_data(canvas_data)
            .configure(routes::configure),
    )
    .await;

    // Empty prompt should fail with 400
    let payload = EvaluateDecisionRequest {
        model: "mock-model".to_string(),
        model_type: DecisionModelType::Choice,
        instruction: "Select best".to_string(),
        prompt: "   ".to_string(),
        images: vec![],
        choices: Some(vec![]),
        score_levels: None,
        noul_criteria: None,
    };

    let req = test::TestRequest::post()
        .uri("/canvas/evaluate")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 400);
}
