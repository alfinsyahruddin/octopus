use actix_web::{get, web, HttpResponse};

use crate::{
    entities::{
        app_response::AppResponse,
        base_response::{BaseResponse, JsonFromStringTrait},
    },
    routes::canvas_route::canvas_scope,
};

pub mod canvas_route;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(health).service(canvas_scope());
}

#[get("/")]
pub async fn index() -> AppResponse<String> {
    String::from("Octopus API service running").json_data()
}

#[get("/health")]
pub async fn health() -> AppResponse<String> {
    String::from("ok").json_data()
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(BaseResponse::<()>::error(404, "Not found"))
}
