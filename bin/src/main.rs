use axum::{
    routing::get,
    Router,
};

use routes::{google_login::{self, google_login}, health};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/health", get(health::health))
        .route("/google/auth/login", get(google_login::google_login));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server Running");
    axum::serve(listener, app).await.unwrap();
}