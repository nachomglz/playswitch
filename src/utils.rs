use std::env::var;
use reqwest::{self, header::{self, HeaderMap, HeaderValue}};

pub fn check_env() -> bool {
    var("SPOTIFY_CLIENT_ID").is_ok()
        && var("SPOTIFY_CLIENT_SECRET").is_ok()
        && var("SPOTIFY_API_BASE_URL").is_ok()
}

pub async fn get_current_user_id() -> Result<String, Box<dyn std::error::Error>> {
    // the token should exist already
    let token = var("SPOTIFY_TOKEN").unwrap();
    let base_uri = var("SPOTIFY_API_BASE_URL").unwrap();

    let uri = format!("{}/me", base_uri);
    let mut headers = HeaderMap::new();
    headers
        .insert(header::AUTHORIZATION, HeaderValue::from_str(&format!("Bearer: {}", token)).unwrap());

    println!("Request uri: {}", uri);

    let request = reqwest::Client::builder().build().unwrap();
    let response = request
        .get(uri)
        .headers(headers)
        .send()
    .await?;

    println!("response: {:?}", response);


    Ok("useriduserisuseriuseriduserisuseriuseriduserisuserisss".to_owned())
}
