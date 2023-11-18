mod save_file;

use axum::{extract::DefaultBodyLimit, routing::post, Router};
use save_file::save_file;

pub fn create_router() -> Router {
    Router::new()
        .route(
            "/save_file",
            post(save_file).route_layer(DefaultBodyLimit::max(135476000)),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
