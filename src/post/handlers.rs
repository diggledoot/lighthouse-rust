use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use hyper::StatusCode;
use orm::{
    entities::post,
    sea_orm::{ActiveModelTrait, EntityTrait},
    entities::prelude::Post
};

use crate::app_state::AppState;

pub async fn create_post(
    State(_state): State<AppState>,
    Json(post): Json<post::Model>,
) -> impl IntoResponse {
    let db = &_state.conn;
    let post_active_model = post::ActiveModel {
        post_title: orm::sea_orm::ActiveValue::Set(post.post_title.clone()),
        post_content: orm::sea_orm::ActiveValue::Set(post.post_content.clone()),
        ..Default::default()
    };
    let res = post_active_model.insert(db).await;
    let Ok(db_post) = res else{
        return (StatusCode::NOT_FOUND,Json(None));
    };
    (StatusCode::OK,Json(Some(db_post)))
}

pub async fn get_post_by_id(Path(id): Path<u32>, State(_state): State<AppState>) -> impl IntoResponse {
    let db = &_state.conn;
    let res = Post::find_by_id(id as i32).one(db).await;
    let Ok(optional_post) = res else{
        return (StatusCode::NOT_FOUND,Json(None));
    };
    let Some(post) = optional_post else{
        return (StatusCode::NOT_FOUND,Json(None));
    };
    (StatusCode::OK,Json(Some(post)))
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
