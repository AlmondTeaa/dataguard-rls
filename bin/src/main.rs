use axum::{
    extract::State,
    routing::get,
    http::HeaderMap,
    Json, Router
};
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions,FromRow};
use std::net::SocketAddr;


use routes::{google_login, health,google_callback, get_html};

#[derive(Serialize, FromRow)]
struct Employee {
    id: i32,
    name: String,
    department: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://app_user:123@localhost:5432/rlsdemo")
        .await
        .unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/health", get(health::health))
        .route("/google/auth/login", get(google_login::google_login))
        .route("/auth/google/callback", get(google_callback::google_callback))
        .route("/index", get(get_html::index))
        .route("/dashboard", get(get_html::dashboard))
        .route("/popup", get(get_html::popup))
        .route("/api/employees", get(get_employees)).with_state(pool);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server Running");
    axum::serve(listener, app).await.unwrap();
}

async fn get_employees(
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Json<Vec<Employee>> {

    let token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");

    println!("Token: {}", token);

    let mut tx = pool.begin().await.unwrap();

    // 🔥 FIXED LINE HERE
    sqlx::query(&format!(
        "SET LOCAL app.user_id = '{}'",
        token
    ))
    .execute(&mut *tx)
    .await
    .unwrap();

    let rows = sqlx::query_as::<_, Employee>(
        r#"
        SELECT id, name, department
        FROM employees
        "#
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();

    Json(rows)
}