use axum::{extract::Path, Json};
use orm::entities::post;

pub async fn create_post(Json(payload): Json<post::Model>) -> String {
    format!("create post with the following json {:?}", payload)
}

pub async fn get_post_by_id(Path(id): Path<u32>) -> String {
    format!("get on post with an id of {}", id)
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
