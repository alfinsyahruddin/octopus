use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::entities::{app_config::AppConfig, app_error::AppError};

pub async fn setup_db(config: &AppConfig) -> Result<PgPool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&pool).await.map_err(|error| {
        eprintln!("database migration error: {error}");
        AppError::Internal
    })?;

    Ok(pool)
}
