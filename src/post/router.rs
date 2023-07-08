use crate::fallback;
use axum::Router;

pub fn base_router() -> Router {
    Router::new().fallback(fallback)
}
