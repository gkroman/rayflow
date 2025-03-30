use axum::{extract::Path, response::IntoResponse, Json};
use std::fs;

pub async fn get_canvases() -> impl IntoResponse {
    let paths = fs::read_dir("data").unwrap();
    let canvases: Vec<String> = paths
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("canvas_"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    Json(canvases)
}

pub async fn get_canvas(Path(id): Path<String>) -> impl IntoResponse {
    let path = format!("data/canvas_{}.json", id);
    let data = fs::read_to_string(path).unwrap_or("{}".to_string());
    axum::response::Html(data)
}

