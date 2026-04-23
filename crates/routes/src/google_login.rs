use std::env;

use axum::response::Redirect;

pub async fn google_login() -> Redirect{
    dotenvy::dotenv().expect("Error");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found in .env");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI not found in .env");

       let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth\
        ?client_id={}&redirect_uri={}&response_type=code\
        &scope=openid%20email%20profile\
        &access_type=offline&prompt=consent",
        client_id,
        redirect_uri
    );
    println!("Redirecting to Google Login");
    Redirect::temporary(&url)
}