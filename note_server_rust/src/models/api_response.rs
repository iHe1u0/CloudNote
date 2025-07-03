use serde::{Deserialize, Serialize};

use crate::models::api_code::ApiCode;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

#[allow(dead_code)]
impl<T> ApiResponse<T> {
    pub fn success_msg(message: &str) -> Self {
        ApiResponse {
            code: ApiCode::Success.code(),
            message: message.to_string(),
            data: None,
        }
    }
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: ApiCode::Success.code(),
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn success_data(message: &str, data: T) -> Self {
        ApiResponse {
            code: ApiCode::Success.code(),
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: &str) -> Self {
        ApiResponse {
            code,
            message: message.to_string(),
            data: None,
        }
    }
}
