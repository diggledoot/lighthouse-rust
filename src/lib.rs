use axum::Router;
use hyper::{StatusCode, Uri};

pub fn create_base_router() -> Router {
    Router::new().fallback(fallback)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
