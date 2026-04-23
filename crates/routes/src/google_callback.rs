use std::env;

use axum::{extract::Query, response::{IntoResponse, Redirect}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleCallback {
    code: String
}

pub async fn google_callback(Query(params):Query<GoogleCallback>) -> Redirect{
    println!("Callback Starting");
    dotenvy::dotenv().expect("Error");
    let client_id = env::var("CLIENT_ID").expect("NOT FOUND");
    let client_secret = env::var("CLIENT_SECRET").expect("NOT FOUND");
    let redirect_uri = env::var("REDIRECT_URI").expect("NOT FOUND");
    let client = reqwest::Client::new();

    // Exchange code for token
    let token_res: serde_json::Value = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", params.code.as_str()),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("redirect_uri", &redirect_uri),
            ("grant_type", "authorization_code"),
        ])
        .send().await.unwrap()
        .json().await.unwrap();

    //println!("Token Response: {:?}",&token_res);

    let access_token = token_res["access_token"]
        .as_str()
        .expect("No access token");

    // Fetch user info
    let user_res: serde_json::Value = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo").bearer_auth(access_token)
        .send().await.unwrap()
        .json().await.unwrap();

    println!("User info: {:?}", user_res);
    Redirect::to(&format!("http://localhost:8080/popup?token={}", user_res["id"].as_str().unwrap().to_string()))
}