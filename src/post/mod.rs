pub mod post_handlers;
pub mod post_router;

use self::post_router::base_router;
use axum::{
    routing::{get, post},
    Router,
};
use post_handlers::*;

pub fn post_router() -> Router {
    let post_routes = Router::new()
        .route("/create", post(create_post))
        .route("/:id", get(get_post));
    base_router(post_routes)
}
