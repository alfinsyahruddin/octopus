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
