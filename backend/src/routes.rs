use axum::{routing::get, Router};

use crate::handlers::{canvas::*, nodes::*, websocket::*};

pub fn app() -> Router {
    Router::new()
        .route("/nodes", get(get_nodes))
        .route("/canvases", get(get_canvases))
        .route("/canvas/:id", get(get_canvas))
        .route("/sync", get(ws_handler))
}
