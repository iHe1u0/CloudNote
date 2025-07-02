use crate::auth::{create_jwt, hash_password, verify_password};
use crate::models::user::User;
use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;
use sqlx::MySqlPool;

pub fn routes() -> Router<MySqlPool> {
    Router::new()
        .route("/note/api/register", post(register))
        .route("/note/api/login", post(login))
}

#[derive(Deserialize)]
struct AuthPayload {
    email: String,
    password: String,
}

async fn register(
    State(pool): State<MySqlPool>,
    Json(payload): Json<AuthPayload>,
) -> Json<serde_json::Value> {
    let password_hash = hash_password(&payload.password).unwrap();

    let _ = sqlx::query("INSERT INTO users (email, password_hash) VALUES (?, ?)")
        .bind(&payload.email)
        .bind(password_hash)
        .execute(&pool)
        .await
        .unwrap();

    Json(serde_json::json!({"message": "注册成功"}))
}

async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<AuthPayload>,
) -> Json<serde_json::Value> {
    println!("Login attempt for email: {}", payload.email);
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_one(&pool)
        .await
        .unwrap();

    if verify_password(&payload.password, &user.password_hash).unwrap() {
        let token = create_jwt(user.id).unwrap();
        Json(serde_json::json!({"token": token}))
    } else {
        Json(serde_json::json!({"message": "密码错误"}))
    }
}
