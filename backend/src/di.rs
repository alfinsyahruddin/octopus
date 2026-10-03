use actix_web::web::{Data, ServiceConfig};
use reqwest::Client;
use std::sync::Arc;

use crate::{
    entities::{app_config::AppConfig, app_error::AppError},
    services::{canvas_service::CanvasService, ollama_engine::OllamaEngine},
    setup::setup_http_client::setup_http_client,
    traits::decision_engine::DecisionEngine,
};

#[derive(Clone)]
pub struct AppDependencies {
    pub config: Data<AppConfig>,
    pub http_client: Data<Client>,
    pub canvas_service: Data<CanvasService>,
}

impl AppDependencies {
    pub async fn build(config: AppConfig) -> Result<Self, AppError> {
        let http_client = setup_http_client()?;

        let clef_engine: Arc<dyn DecisionEngine> = Arc::new(OllamaEngine::new(
            http_client.clone(),
            config.ollama_base_url.clone(),
            "clef-flash",
        ));

        let canvas_service = CanvasService::new(vec![clef_engine], "clef-flash");

        Ok(Self {
            config: Data::new(config),
            http_client: Data::new(http_client),
            canvas_service: Data::new(canvas_service),
        })
    }

    pub fn configure(&self, cfg: &mut ServiceConfig) {
        cfg.app_data(self.config.clone())
            .app_data(self.http_client.clone())
            .app_data(self.canvas_service.clone());
    }
}
