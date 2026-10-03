use redis::aio::ConnectionManager;
use crate::entities::{app_config::AppConfig, app_error::AppError};

pub async fn setup_redis(config: &AppConfig) -> Result<ConnectionManager, AppError> {
    let client = redis::Client::open(config.redis_url.clone()).map_err(AppError::from)?;
    ConnectionManager::new(client).await.map_err(AppError::from)
}
