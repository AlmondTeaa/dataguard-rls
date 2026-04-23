use axum::response::Html;
use std::fs;

pub async fn index() -> Html<String> {
    Html(fs::read_to_string("view/index.html").unwrap())
}

pub async fn dashboard() -> Html<String> {
    Html(fs::read_to_string("view/dashboard.html").unwrap())
}

pub async fn popup() -> Html<String> {
    Html(std::fs::read_to_string("view/popup.html").unwrap())
}

