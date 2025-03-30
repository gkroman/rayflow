use axum::response::IntoResponse;
use axum::response::Html;
use std::fs;

pub async fn get_nodes() -> impl IntoResponse {
    let data = fs::read_to_string("data/nodes.json").unwrap_or("[]".to_string());
    Html(data)
}

