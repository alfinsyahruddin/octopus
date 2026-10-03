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
