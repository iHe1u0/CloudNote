use crate::utils::string_util::StringUtils;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

impl AuthPayload {
    pub fn get_valid_table_name(&self) -> String {
        StringUtils::get_valid_table_name(&self.email)
    }
}
