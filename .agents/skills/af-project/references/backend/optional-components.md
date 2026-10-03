# Backend Optional Components: Auth, Sessions, & Caching

These optional components provide production-grade authentication, password hashing, and revocable Redis session management. Add them only when project requirements include user accounts and stateful sessions.

---

## 1. Domain Constants, Roles, & Tokens

### `constants/mod.rs`
Store Redis key prefixes and token lifespans centrally:

```rust
pub const SESSION_KEY_PREFIX: &str = "auth:session";
pub const USER_SESSIONS_KEY_PREFIX: &str = "auth:user-sessions";
pub const ACCESS_TOKEN_EXPIRATION_SECONDS: u64 = 900;       // 15 minutes
pub const REFRESH_TOKEN_EXPIRATION_SECONDS: u64 = 604800;   // 7 days
```

### `enums/user_role.rs`
Map database roles to typed enums:

```rust
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum UserRole {
    Admin,
    Member,
}

impl UserRole {
    pub const fn is_admin(self) -> bool {
        matches!(self, Self::Admin)
    }
}
```

### `enums/token_type.rs`
Distinguish access tokens from refresh tokens:

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Access,
    Refresh,
}
```

---

## 2. Password Hashing & JWT Lifecycle

### Password Hashing (`helpers/hash_helper.rs`)
Hash and verify user passwords using Argon2id with cryptographically secure random salts:

```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::entities::app_error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| {
            eprintln!("Password hashing error: {e}");
            AppError::Internal
        })
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|_| AppError::Internal)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
```

### JWT Creation & Verification (`helpers/token_helper.rs`)
Encode and decode HMAC-SHA256 tokens carrying claims and session UUIDs:

```rust
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    entities::{app_config::AppConfig, app_error::AppError},
    enums::{token_type::TokenType, user_role::UserRole},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: UserRole,
    pub token_type: TokenType,
    pub sid: Uuid,
    pub iat: i64,
    pub exp: i64,
}

pub fn decode_token(token: &str, config: &AppConfig) -> Result<Claims, AppError> {
    let bearer_token = token.trim().strip_prefix("Bearer ").unwrap_or(token.trim());
    decode::<Claims>(
        bearer_token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|token| token.claims)
    .map_err(|_| AppError::unauthorized("Invalid or expired token"))
}
```

> [!CAUTION]
> Never log raw passwords, access tokens, or password hashes in server log streams or error traces.

---

## 3. Revocable Redis Session Tracking

> [!IMPORTANT]
> **Multi-Device Revocation Invariant**: When a user changes their password, changes roles, or deletes their account, the application must immediately revoke every active session across all devices.

To enable multi-device session revocation:
1. Store session ID → user ID mapping at `auth:session:{session_id}`.
2. Maintain a set of active session IDs per user at `auth:user-sessions:{user_id}`.

### `setup/setup_redis.rs`
```rust
use redis::aio::ConnectionManager;
use crate::entities::{app_config::AppConfig, app_error::AppError};

pub async fn setup_redis(config: &AppConfig) -> Result<ConnectionManager, AppError> {
    let client = redis::Client::open(config.redis_url.clone()).map_err(AppError::from)?;
    ConnectionManager::new(client).await.map_err(AppError::from)
}
```

### `services/session_service.rs`
```rust
use redis::{aio::ConnectionManager, AsyncCommands};
use uuid::Uuid;

use crate::{
    constants::{SESSION_KEY_PREFIX, USER_SESSIONS_KEY_PREFIX},
    entities::app_error::AppError,
};

#[derive(Clone)]
pub struct SessionService {
    redis: ConnectionManager,
    refresh_ttl_seconds: u64,
}

impl SessionService {
    pub fn new(redis: ConnectionManager, refresh_ttl_seconds: u64) -> Self {
        Self {
            redis,
            refresh_ttl_seconds,
        }
    }

    pub async fn create_session(&self, user_id: Uuid) -> Result<Uuid, AppError> {
        let session_id = Uuid::new_v4();
        let mut connection = self.redis.clone();
        let ttl = i64::try_from(self.refresh_ttl_seconds).map_err(|_| AppError::Internal)?;
        let session_key = format!("{SESSION_KEY_PREFIX}:{session_id}");
        let user_key = format!("{USER_SESSIONS_KEY_PREFIX}:{user_id}");

        let _: () = connection
            .set_ex(&session_key, user_id.to_string(), self.refresh_ttl_seconds)
            .await?;
        let _: usize = connection.sadd(&user_key, session_id.to_string()).await?;
        let _: bool = connection.expire(&user_key, ttl).await?;
        Ok(session_id)
    }

    pub async fn validate_session(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, AppError> {
        let mut connection = self.redis.clone();
        let session_key = format!("{SESSION_KEY_PREFIX}:{session_id}");
        let subject: Option<String> = connection.get(session_key).await?;
        Ok(subject.is_some_and(|stored_user_id| stored_user_id == user_id.to_string()))
    }

    pub async fn revoke_session(&self, session_id: Uuid) -> Result<(), AppError> {
        let mut connection = self.redis.clone();
        let session_key = format!("{SESSION_KEY_PREFIX}:{session_id}");
        let user_id: Option<String> = connection.get(&session_key).await?;
        let _: usize = connection.del(&session_key).await?;

        if let Some(user_id) = user_id {
            let user_key = format!("{USER_SESSIONS_KEY_PREFIX}:{user_id}");
            let _: usize = connection.srem(user_key, session_id.to_string()).await?;
        }
        Ok(())
    }

    pub async fn revoke_all_user_sessions(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut connection = self.redis.clone();
        let user_key = format!("{USER_SESSIONS_KEY_PREFIX}:{user_id}");
        let session_ids: Vec<String> = connection.smembers(&user_key).await?;
        for session_id in session_ids {
            let session_key = format!("{SESSION_KEY_PREFIX}:{session_id}");
            let _: usize = connection.del(session_key).await?;
        }
        let _: usize = connection.del(user_key).await?;
        Ok(())
    }
}
```

---

## 4. Standard Backend Dependencies (`Cargo.toml`)

Standard dependency manifest and release profile when building an Actix API with PostgreSQL, Redis, and Auth:

```toml
[dependencies]
actix-web = "4.9"
actix-cors = "0.7"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate"] }
redis = { version = "0.27", features = ["tokio-comp", "connection-manager"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
thiserror = "2"
validator = { version = "0.21", features = ["derive"] }
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
dotenvy = "0.15"
argon2 = "0.5"
jsonwebtoken = "9.3"
async-trait = "0.1"

[profile.release]
opt-level = 3          # Highest optimization for speed
lto = "fat"            # Full link-time optimization across crate boundaries
codegen-units = 1      # Maximize compiler optimization passes
panic = "abort"        # Eliminates unwinding landing pads for smaller binary
strip = "symbols"      # Strips debug symbols and dead code
```
