use crate::auth::{create_jwt, hash_password, verify_password};
use crate::models::api_code::ApiCode;
use crate::models::api_response::ApiResponse;
use crate::models::user::User;
use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;
use sqlx::MySqlPool;

pub fn routes() -> Router<MySqlPool> {
    Router::new()
        .route("/note/api/register", post(register))
        .route("/note/api/login", post(login))
}

#[derive(Deserialize, Debug)]
struct AuthPayload {
    email: String,
    password: String,
}

async fn register(
    State(pool): State<MySqlPool>,
    Json(payload): Json<AuthPayload>,
) -> Json<ApiResponse<()>> {
    // 先判断用户是否存在
    let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await;

    match &existing_user {
        Ok(Some(_)) => {
            return Json(ApiResponse::error(ApiCode::UserExists.code(), "用户已存在"));
        }
        Ok(None) => println!("没有查询到用户"),
        Err(e) => {
            eprintln!("Error checking existing user: {:?}", e);
            return Json(ApiResponse::error(
                ApiCode::OperationFailed.code(),
                "查询用户失败",
            ));
        }
    }

    let password_hash = hash_password(&payload.password).unwrap();

    let result = sqlx::query("INSERT INTO users (email, password_hash) VALUES (?, ?)")
        .bind(&payload.email)
        .bind(password_hash)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => {
            return Json(ApiResponse::success_msg("注册成功"));
        }
        Err(e) => {
            eprintln!("Error registering user: {:?}", e);
            return Json(ApiResponse::error(
                ApiCode::OperationFailed.code(),
                "注册失败",
            ));
        }
    }
}

async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<AuthPayload>,
) -> Json<ApiResponse<String>> {

    println!("Login attempt with email: {}", payload.email);

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_one(&pool)
        .await;

    match user {
        Ok(user) => {
            if verify_password(&payload.password, &user.password_hash).unwrap() {
                let token = create_jwt(user.id).unwrap();
                Json(ApiResponse::success_data("登录成功", token))
            } else {
                Json(ApiResponse::error(ApiCode::AuthFailed.code(), "密码错误"))
            }
        }
        Err(_) => Json(ApiResponse::error(
            ApiCode::UserNotActivated.code(),
            "用户不存在",
        )),
    }
}
