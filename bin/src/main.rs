use axum::{
    routing::get,
    Router,
};

use routes::{google_login, health,google_callback, get_html};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/health", get(health::health))
        .route("/google/auth/login", get(google_login::google_login))
        .route("/auth/google/callback", get(google_callback::google_callback))
        .route("/index", get(get_html::index))
        .route("/dashboard", get(get_html::dashboard))
        .route("/popup", get(get_html::popup));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server Running");
    axum::serve(listener, app).await.unwrap();
}