use actix_web::web::{self, Data, ServiceConfig};
use reqwest::Client;
use sqlx::PgPool;

use crate::{
    entities::{app_config::AppConfig, app_error::AppError},
    setup::{setup_db::setup_db, setup_http_client::setup_http_client},
};

#[derive(Clone)]
pub struct AppDependencies {
    pub config: Data<AppConfig>,
    pub db: Data<PgPool>,
    pub http_client: Data<Client>,
    // Add additional repositories and services here
}

impl AppDependencies {
    pub async fn build(config: AppConfig) -> Result<Self, AppError> {
        let db = setup_db(&config).await?;
        let http_client = setup_http_client()?;

        Ok(Self {
            config: Data::new(config),
            db: Data::new(db),
            http_client: Data::new(http_client),
        })
    }

    pub fn configure(&self, cfg: &mut ServiceConfig) {
        cfg.app_data(self.config.clone())
            .app_data(self.db.clone())
            .app_data(self.http_client.clone());
    }
}
