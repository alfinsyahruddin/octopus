use reqwest::Client;
use std::time::Duration;

use crate::entities::app_error::AppError;

pub fn setup_http_client() -> Result<Client, AppError> {
    Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|error| {
            eprintln!("HTTP client initialization error: {error}");
            AppError::internal(format!("HTTP client build failed: {error}"))
        })
}
