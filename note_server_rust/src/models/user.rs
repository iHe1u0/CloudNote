use crate::utils::string_util::StringUtils;
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

impl User {
    pub fn make_valid(&mut self) {
        self.email = self.email.trim().to_string();
    }
    /// 获取用户的有效表名
    pub fn get_valid_table_name(&self) -> String {
        StringUtils::get_valid_table_name(&self.email)
    }
}
