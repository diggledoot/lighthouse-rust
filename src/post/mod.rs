use axum::Router;

use self::router::base_router;

pub mod router;

pub fn post_router() -> Router {
    base_router()
}
