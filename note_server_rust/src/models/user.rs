use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}
