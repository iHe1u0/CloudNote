use crate::auth::{create_jwt, hash_password, verify_password};
use crate::models::api_code::ApiCode;
use crate::models::api_response::ApiResponse;
use crate::models::auth_payload::AuthPayload;
use crate::models::user::User;
use axum::{Json, Router, extract::State, routing::post};
use sqlx::{MySql, MySqlPool, Transaction};

pub fn routes() -> Router<MySqlPool> {
    Router::new().route("/note/api/register", post(register)).route("/note/api/login", post(login))
}

async fn register(State(pool): State<MySqlPool>, Json(payload): Json<AuthPayload>) -> Json<ApiResponse<()>> {
    let mut tx: Transaction<'_, MySql> = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Error starting transaction: {:?}", e);
            return Json(ApiResponse::error(ApiCode::OperationFailed.code(), "注册失败"));
        }
    };

    // 检查是否存在
    let existing_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email.trim())
        .fetch_optional(&mut *tx) // 注意事务要传引用
        .await;

    if let Ok(Some(_)) = existing_user {
        return Json(ApiResponse::error(ApiCode::UserExists.code(), "用户已存在"));
    } else if let Err(e) = existing_user {
        eprintln!("Error checking existing user: {:?}", e);
        return Json(ApiResponse::error(ApiCode::OperationFailed.code(), "查询用户失败"));
    }

    let valid_table_name = payload.get_valid_table_name();

    let password_hash = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Password hash error: {:?}", e);
            return Json(ApiResponse::error(ApiCode::OperationFailed.code(), "注册失败"));
        }
    };

    // 插入用户
    let result = sqlx::query("INSERT INTO users (email, password_hash) VALUES (?, ?)")
        .bind(&payload.email.trim())
        .bind(password_hash)
        .execute(&mut *tx)
        .await;

    if let Err(e) = result {
        eprintln!("Error inserting user: {:?}", e);
        return Json(ApiResponse::error(ApiCode::OperationFailed.code(), "注册失败"));
    }

    // 创建用户表
    let create_table_sql =
        format!("CREATE TABLE `{}` (id INT PRIMARY KEY AUTO_INCREMENT, data TEXT)", valid_table_name);

    if let Err(e) = sqlx::query(&create_table_sql).execute(&mut *tx).await {
        eprintln!("Error creating user table: {:?}", e);
        return Json(ApiResponse::error(ApiCode::OperationFailed.code(), "注册失败"));
    }

    // 一切正常 -> 提交事务
    if let Err(e) = tx.commit().await {
        eprintln!("Error committing transaction: {:?}", e);
        return Json(ApiResponse::error(ApiCode::OperationFailed.code(), "注册失败"));
    }

    Json(ApiResponse::success_msg("注册成功"))
}

async fn login(State(pool): State<MySqlPool>, Json(payload): Json<AuthPayload>) -> Json<ApiResponse<String>> {
    let user =
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?").bind(&payload.email).fetch_one(&pool).await;

    match user {
        Ok(mut user) => {
            user.make_valid();
            if verify_password(&payload.password, &user.password_hash).unwrap() {
                let token = create_jwt(user.id).unwrap();
                Json(ApiResponse::success_data("登录成功", token))
            } else {
                Json(ApiResponse::error(ApiCode::AuthFailed.code(), "密码错误"))
            }
        }
        Err(_) => Json(ApiResponse::error(ApiCode::UserNotActivated.code(), "用户不存在")),
    }
}
