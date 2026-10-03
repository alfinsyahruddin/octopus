# Backend Vertical Slice Feature

This reference implementation demonstrates how to build a complete vertical slice through every backend layer: Entities, Repository, Service, Route, Guard, Client adapter, Migration, and HTTP Contract test.

---

## 1. Domain DTOs (`entities/app_settings.rs`)

Define strongly typed request and response DTOs for the feature:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppSettingsRequest {
    pub maintenance_mode: bool,
    pub system_announcement: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettingsResponse {
    pub maintenance_mode: bool,
    pub system_announcement: Option<String>,
}
```

---

## 2. PostgreSQL Queries (`repositories/settings_repository.rs`)

Repositories own raw database interactions. Use parameterized queries and handle SQL errors:

```rust
use sqlx::PgPool;
use crate::entities::app_error::AppError;
use crate::entities::app_settings::AppSettingsResponse;

#[derive(Clone)]
pub struct SettingsRepository {
    db: PgPool,
}

impl SettingsRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_settings(&self) -> Result<AppSettingsResponse, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT maintenance_mode, system_announcement
            FROM app_settings
            WHERE id = 1
            "#
        )
        .fetch_optional(&self.db)
        .await
        .map_err(AppError::from)?;

        match row {
            Some(r) => Ok(AppSettingsResponse {
                maintenance_mode: r.maintenance_mode,
                system_announcement: r.system_announcement,
            }),
            None => Ok(AppSettingsResponse {
                maintenance_mode: false,
                system_announcement: None,
            }),
        }
    }

    pub async fn update_settings(
        &self,
        maintenance_mode: bool,
        system_announcement: Option<String>,
    ) -> Result<AppSettingsResponse, AppError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO app_settings (id, maintenance_mode, system_announcement, updated_at)
            VALUES (1, $1, $2, CURRENT_TIMESTAMP)
            ON CONFLICT (id) DO UPDATE
            SET maintenance_mode = EXCLUDED.maintenance_mode,
                system_announcement = EXCLUDED.system_announcement,
                updated_at = CURRENT_TIMESTAMP
            RETURNING maintenance_mode, system_announcement
            "#,
            maintenance_mode,
            system_announcement
        )
        .fetch_one(&self.db)
        .await
        .map_err(AppError::from)?;

        Ok(AppSettingsResponse {
            maintenance_mode: row.maintenance_mode,
            system_announcement: row.system_announcement,
        })
    }
}
```

---

## 3. Business Logic Orchestration (`services/settings_service.rs`)

The service validates inputs, coordinates repositories, and invalidates or updates cache:

```rust
use redis::aio::ConnectionManager;
use crate::entities::app_error::AppError;
use crate::entities::app_settings::{AppSettingsResponse, UpdateAppSettingsRequest};
use crate::repositories::settings_repository::SettingsRepository;

#[derive(Clone)]
pub struct SettingsService {
    repo: SettingsRepository,
    redis: Option<ConnectionManager>,
}

impl SettingsService {
    pub fn new(repo: SettingsRepository, redis: Option<ConnectionManager>) -> Self {
        Self { repo, redis }
    }

    pub async fn get_settings(&self) -> Result<AppSettingsResponse, AppError> {
        self.repo.get_settings().await
    }

    pub async fn update_settings(
        &self,
        req: UpdateAppSettingsRequest,
    ) -> Result<AppSettingsResponse, AppError> {
        let updated = self.repo.update_settings(req.maintenance_mode, req.system_announcement).await?;

        // Invalidate or update cache if Redis is configured
        if let Some(mut redis) = self.redis.clone() {
            let cache_key = "app:settings:maintenance";
            let val = if updated.maintenance_mode { "1" } else { "0" };
            let _: Result<(), _> = redis::cmd("SET")
                .arg(cache_key)
                .arg(val)
                .query_async(&mut redis)
                .await;
        }

        Ok(updated)
    }
}
```

---

## 4. HTTP Boundary (`routes/settings_route.rs`)

Handlers extract parameters, invoke services, and convert results into `AppResponse<T>`:

```rust
use actix_web::{get, patch, web::{Data, Json}};
use crate::entities::app_response::{AppResponse, IntoResponseTrait};
use crate::entities::app_settings::{AppSettingsResponse, UpdateAppSettingsRequest};
use crate::guards::{AuthenticatedUser, RequireAdmin};
use crate::services::settings_service::SettingsService;

#[get("/api/settings")]
pub async fn get_settings(
    service: Data<SettingsService>,
    _user: AuthenticatedUser,
) -> AppResponse<AppSettingsResponse> {
    service.get_settings().await.json()
}

#[patch("/api/settings")]
pub async fn update_settings(
    service: Data<SettingsService>,
    _user: AuthenticatedUser,
    _admin: RequireAdmin,
    body: Json<UpdateAppSettingsRequest>,
) -> AppResponse<AppSettingsResponse> {
    service.update_settings(body.into_inner()).await.json()
}
```

---

## 5. Authorization Guards (`guards/authenticated_user.rs` & `guards/require_admin.rs`)

Extractors authenticate credentials, validate active Redis sessions, and enforce role requirements before route handlers execute:

### `guards/authenticated_user.rs`
```rust
use std::{future::Future, pin::Pin};
use actix_web::{dev::Payload, http::header::AUTHORIZATION, web::Data, FromRequest, HttpRequest};

use crate::{
    entities::{app_config::AppConfig, app_error::AppError},
    enums::token_type::TokenType,
    helpers::token_helper::{decode_token, Claims},
    services::session_service::SessionService,
};

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub claims: Claims,
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = request.clone();
        Box::pin(async move { authenticate_request(&request).await })
    }
}

pub async fn authenticate_request(request: &HttpRequest) -> Result<AuthenticatedUser, AppError> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::unauthorized("Authorization token is required"))?;

    let config = request
        .app_data::<Data<AppConfig>>()
        .ok_or(AppError::Internal)?;
    let sessions = request
        .app_data::<Data<SessionService>>()
        .ok_or(AppError::Internal)?;

    let claims = decode_token(token, config.get_ref())?;
    if claims.token_type != TokenType::Access {
        return Err(AppError::unauthorized("Access token required"));
    }

    if !sessions.validate_session(claims.sid, claims.sub).await? {
        return Err(AppError::unauthorized("Session is no longer active"));
    }

    Ok(AuthenticatedUser { claims })
}
```

### `guards/require_admin.rs`
```rust
use std::{future::Future, pin::Pin};
use actix_web::{dev::Payload, FromRequest, HttpRequest};

use crate::{
    entities::app_error::AppError,
    guards::authenticated_user::{authenticate_request, AuthenticatedUser},
};

#[derive(Debug)]
pub struct RequireAdmin {
    pub user: AuthenticatedUser,
}

impl FromRequest for RequireAdmin {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = request.clone();
        Box::pin(async move {
            let user = authenticate_request(&request).await?;
            if !user.claims.role.is_admin() {
                return Err(AppError::Forbidden("Administrator access is required".into()));
            }
            Ok(Self { user })
        })
    }
}
```

---

## 6. External Adapter Seam (`clients/external_client.rs`)

Isolate outbound HTTP calls behind an async trait so services can be tested with fakes/mocks without external network access:

```rust
use async_trait::async_trait;
use reqwest::Client;
use crate::entities::app_error::AppError;

#[async_trait]
pub trait ExternalClientTrait: Send + Sync {
    async fn fetch_data(&self, endpoint: &str) -> Result<String, AppError>;
}

#[derive(Clone)]
pub struct ExternalClient {
    http: Client,
    base_url: String,
}

impl ExternalClient {
    pub fn new(http: Client, base_url: String) -> Self {
        Self { http, base_url }
    }
}

#[async_trait]
impl ExternalClientTrait for ExternalClient {
    async fn fetch_data(&self, endpoint: &str) -> Result<String, AppError> {
        let url = format!("{}{}", self.base_url, endpoint);
        let res = self.http.get(&url).send().await.map_err(|e| {
            eprintln!("Outbound request failed: {e}");
            AppError::Internal
        })?;

        res.text().await.map_err(|_| AppError::Internal)
    }
}
```

---

## 7. Database Migration (`migrations/202609020001_create_app_settings.sql`)

Sequential, append-only migration tracking the feature schema:

```sql
CREATE TABLE IF NOT EXISTS app_settings (
    id INT PRIMARY KEY DEFAULT 1,
    maintenance_mode BOOLEAN NOT NULL DEFAULT FALSE,
    system_announcement TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT single_row CHECK (id = 1)
);

INSERT INTO app_settings (id, maintenance_mode, system_announcement)
VALUES (1, FALSE, NULL)
ON CONFLICT (id) DO NOTHING;
```

---

## 8. HTTP Contract Test (`tests/http_contract.rs`)

Black-box contract test verifying response envelope status code and error formatting:

```rust
use actix_web::{http::StatusCode, test, web, App};
use backend::routes;

#[actix_web::test]
async fn not_found_endpoint_returns_standard_error_envelope() {
    let service = test::init_service(
        App::new()
            .default_service(web::to(routes::not_found))
            .configure(routes::configure),
    )
    .await;

    let response = test::call_service(
        &service,
        test::TestRequest::get()
            .uri("/api/non-existent-endpoint")
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = test::read_body(response).await;
    let envelope: serde_json::Value = serde_json::from_slice(&body).expect("valid response JSON");

    assert_eq!(envelope["status"], 404);
    assert_eq!(envelope["message"], "Not found");
    assert!(envelope["data"].is_null());
    assert!(envelope["timestamp"].is_string());
}
```
