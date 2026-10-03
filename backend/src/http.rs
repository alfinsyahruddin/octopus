use actix_cors::Cors;
use actix_web::{error::InternalError, web, ResponseError};

use crate::entities::app_error::AppError;

pub fn cors(allowed_origin: &str) -> Cors {
    let mut cors = Cors::default()
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
        .max_age(3600);

    for origin in allowed_origin.split(',') {
        let trimmed = origin.trim();
        if !trimmed.is_empty() {
            cors = cors.allowed_origin(trimmed);
        }
    }

    if allowed_origin.contains("localhost:3000") && !allowed_origin.contains("127.0.0.1:3000") {
        cors = cors.allowed_origin("http://127.0.0.1:3000");
    } else if allowed_origin.contains("127.0.0.1:3000")
        && !allowed_origin.contains("localhost:3000")
    {
        cors = cors.allowed_origin("http://localhost:3000");
    }

    cors
}

pub fn json_config() -> web::JsonConfig {
    web::JsonConfig::default().error_handler(|_, _| {
        InternalError::from_response(
            "invalid JSON body",
            AppError::bad_request("Invalid JSON request body").error_response(),
        )
        .into()
    })
}
