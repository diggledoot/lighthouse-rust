use axum::{extract::Path, Json};
use orm::entities::post;

//c
pub async fn create_post(Json(payload): Json<post::Model>) -> String {
    format!("create post with the following json {:?}", payload)
}
//r
pub async fn get_post(Path(id): Path<u32>) -> String {
    format!("get on post with an id of {}", id)
}
