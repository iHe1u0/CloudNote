use axum::{Router, serve};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod auth;
mod db;
mod errors;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    // 加载 .env
    dotenv().ok();

    // 获取数据库连接字符串
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 连接数据库
    let pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Could not connect to database");

    // 一些配置信息
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);
    // 初始化路由
    let app = Router::new()
        .merge(routes::users::routes())
        // .merge(routes::notes::routes())
        .with_state(pool)
        .layer(cors);

    // 绑定监听地址
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("✅ Server running at http://0.0.0.0:3000");

    // 启动服务
    serve(listener, app).await.unwrap();
}
