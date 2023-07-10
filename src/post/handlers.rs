use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};

use hyper::StatusCode;
use orm::{
    entities::post,
    sea_orm::{ActiveModelTrait, EntityTrait},
    entities::{prelude::Post, post::Model}
};

use crate::utilities::app_state::AppState;

pub async fn create_post(
    State(_state): State<AppState>,
    Json(post): Json<post::Model>,
) -> Response {
    let db = &_state.conn;
    let post_active_model = post::ActiveModel {
        post_title: orm::sea_orm::ActiveValue::Set(post.post_title.clone()),
        post_content: orm::sea_orm::ActiveValue::Set(post.post_content.clone()),
        ..Default::default()
    };
    let res = post_active_model.insert(db).await;
    match res{
        Ok(post)=> (StatusCode::OK,Json(Some(post))).into_response(),
        Err(err)=> (StatusCode::NOT_FOUND,Json(Some(err.to_string()))).into_response()
    }
}

pub async fn get_post_by_id(Path(id): Path<i32>, State(_state): State<AppState>) -> Response {
    let db = &_state.conn;
    let res = Post::find_by_id(id).one(db).await;
    if let Err(err) = res {
        return (StatusCode::INTERNAL_SERVER_ERROR,Json(err.to_string())).into_response(); 
    } 
    let Ok(optional_post) = res else{
        return (StatusCode::NOT_FOUND,Json(None::<Model>)).into_response();
    };
    let Some(post) = optional_post else{
        return (StatusCode::NOT_FOUND,Json(None::<Model>)).into_response();
    };
    (StatusCode::OK,Json(Some(post))).into_response()
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
