pub mod post_handler;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use post_handler::*;

pub fn post_router() -> Router {
    Router::new()
        .route("/", post(create_post))
        .route("/:id", get(get_post_by_id))
        .route("/:id", put(update_post_by_id))
        .route("/:id", delete(delete_post_by_id))
}
