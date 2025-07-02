use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};

#[derive(Debug)]
pub enum AppError {
    DbError(sqlx::Error),
    Unauthorized,
    NotFound(String),
    BadRequest(String),
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message): (StatusCode, Value) = match self {
            AppError::DbError(e) => {
                println!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"error": "数据库错误"}),
                )
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, json!({"error": "未授权访问"})),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, json!({"error": msg})),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, json!({"error": msg})),
            AppError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, json!({"error": msg}))
            }
        };

        (status, Json(error_message)).into_response()
    }
}

// 方便将 sqlx::Error 自动转换为 AppError
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DbError(err)
    }
}
