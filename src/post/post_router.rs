use axum::Router;

pub fn base_router(routes: Router) -> Router {
    Router::new().nest("/post", routes)
}
