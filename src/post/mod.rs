pub mod post_handler;
pub mod post_router;

use self::post_router::base_router;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use post_handler::*;

pub fn post_router() -> Router {
    let post_routes = Router::new()
        .route("/", post(create_post))
        .route("/:id", get(get_post_by_id))
        .route("/:id", put(update_post_by_id))
        .route("/:id", delete(delete_post_by_id));
    base_router(post_routes)
}
