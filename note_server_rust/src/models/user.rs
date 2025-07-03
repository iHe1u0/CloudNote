use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    // pub created_at: NaiveDateTime,
    pub created_at: DateTime<Utc>,
}
