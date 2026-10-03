use actix_web::web::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::entities::app_response::AppResponse;

#[derive(Debug, Serialize)]
pub struct BaseResponse<T: Serialize> {
    pub data: Option<T>,
    pub status: u16,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T: Serialize> BaseResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            status: 200,
            message: None,
            timestamp: Utc::now(),
        }
    }
}

impl BaseResponse<()> {
    pub fn message(message: impl Into<String>) -> Self {
        Self {
            data: None,
            status: 200,
            message: Some(message.into()),
            timestamp: Utc::now(),
        }
    }

    pub fn error(status: u16, message: impl Into<String>) -> Self {
        Self {
            data: None,
            status,
            message: Some(message.into()),
            timestamp: Utc::now(),
        }
    }
}

pub trait JsonFromStringTrait {
    fn json(self) -> AppResponse<String>;
    fn json_data(self) -> AppResponse<String>;
}

impl JsonFromStringTrait for String {
    fn json(self) -> AppResponse<String> {
        Ok(Json(BaseResponse {
            data: None,
            status: 200,
            message: Some(self),
            timestamp: Utc::now(),
        }))
    }

    fn json_data(self) -> AppResponse<String> {
        Ok(Json(BaseResponse::success(self)))
    }
}
