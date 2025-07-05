#![allow(dead_code)]
use crate::models::note::Note;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde::Deserialize;
use sqlx::MySqlPool;

pub fn routes() -> Router<MySqlPool> {
    Router::new()
        .route("/note/api/notes", get(list_notes).post(create_note))
        .route("/note/api/notes/:id", get(get_note).put(update_note).delete(delete_note))
}

#[derive(Deserialize)]
struct Pagination {
    page: Option<i64>,
    page_size: Option<i64>,
}

async fn list_notes(State(pool): State<MySqlPool>, Query(pagination): Query<Pagination>) -> Json<Vec<Note>> {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;

    let notes = sqlx::query_as::<_, Note>("SELECT * FROM notes ORDER BY updated_at DESC LIMIT ? OFFSET ?")
        .bind(page_size)
        .bind(offset)
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(notes)
}

async fn create_note(State(pool): State<MySqlPool>, Json(note): Json<Note>) -> Json<serde_json::Value> {
    let _ = sqlx::query("INSERT INTO notes (user_id, title, content) VALUES (?, ?, ?)")
        .bind(note.user_id)
        .bind(&note.title)
        .bind(&note.content)
        .execute(&pool)
        .await
        .unwrap();

    Json(serde_json::json!({"message": "笔记创建成功"}))
}

async fn get_note(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Json<Note> {
    let note = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = ?").bind(id).fetch_one(&pool).await.unwrap();

    Json(note)
}

async fn update_note(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(note): Json<Note>,
) -> Json<serde_json::Value> {
    let _ = sqlx::query("UPDATE notes SET title = ?, content = ? WHERE id = ?")
        .bind(&note.title)
        .bind(&note.content)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    Json(serde_json::json!({"message": "更新成功"}))
}

async fn delete_note(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Json<serde_json::Value> {
    let _ = sqlx::query("DELETE FROM notes WHERE id = ?").bind(id).execute(&pool).await.unwrap();

    Json(serde_json::json!({"message": "删除成功"}))
}
