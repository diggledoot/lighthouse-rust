use axum::{
    extract::{Path, State},
    Json,
};
use orm::{
    entities::post,
    sea_orm::{ActiveModelTrait, ConnectionTrait},
};

use crate::app_state::AppState;

pub async fn create_post(
    State(_state): State<AppState>,
    Json(payload): Json<post::Model>,
) -> String {
    let db = &_state.conn;
    let _ = post::ActiveModel {
        post_title: orm::sea_orm::ActiveValue::Set(payload.post_title.clone()),
        post_content: orm::sea_orm::ActiveValue::Set(payload.post_content.clone()),
        ..Default::default()
    }
    .save(db)
    .await
    .map_err(|err| format!("{:?}", err));
    format!("create post with the following json {:?}", payload)
}

pub async fn get_post_by_id(Path(id): Path<u32>, State(_state): State<AppState>) -> String {
    format!(
        "get on post with an id of {} and using the backend {:?}",
        id,
        _state.conn.get_database_backend()
    )
}

pub async fn update_post_by_id(Path(id): Path<u32>, Json(payload): Json<post::Model>) -> String {
    format!(
        "update post with the following json {:?} of id {}",
        payload, id
    )
}

pub async fn delete_post_by_id(Path(id): Path<u32>) -> String {
    format!("delete post with an id of {}", id)
}
