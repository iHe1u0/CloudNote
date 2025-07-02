use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub category: String,
    pub note_status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
