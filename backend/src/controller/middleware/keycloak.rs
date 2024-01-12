use axum::http::Request;
use axum::{
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use color_eyre::Report;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
struct UserInfo {
    sub: String,
    email_verified: bool,
    name: String,
    preferred_username: String,
    given_name: String,
    family_name: String,
    email: String,
}

pub(crate) async fn auth<B>(
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let token = extract_token(headers).await;
    if token.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let is_valid = verify_token(token.unwrap()).await.unwrap();
    if is_valid {
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn verify_token(token: String) -> Result<bool, Report> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/realms/whats_up/protocol/openid-connect/userinfo")
        .bearer_auth(token)
        .send()
        .await?;

    Ok(response.status().is_success())
}

pub(crate) async fn extract_token(headers: HeaderMap) -> Result<String, StatusCode> {
    let auth_header = headers.get("Authorization");
    if auth_header.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let token = headers.get("Authorization").unwrap().to_str().unwrap();
    let token = token.replace("Bearer ", "");
    Ok(token)
}

pub(crate) async fn get_user_name_from_info(token: String) -> Result<String, Report> {
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:8080/realms/whats_up/protocol/openid-connect/userinfo")
        .bearer_auth(token)
        .send()
        .await?;
    let userinfo = response.json::<UserInfo>().await?;
    Ok(userinfo.preferred_username)
}
