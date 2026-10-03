use actix_web::web::Json;
use serde::Serialize;

use crate::entities::{app_error::AppError, base_response::BaseResponse};

pub type AppResponse<T> = Result<Json<BaseResponse<T>>, AppError>;

pub trait IntoResponseTrait<T: Serialize> {
    fn json(self) -> AppResponse<T>;
}

impl<T: Serialize> IntoResponseTrait<T> for Result<T, AppError> {
    fn json(self) -> AppResponse<T> {
        self.map(BaseResponse::success).map(Json)
    }
}
