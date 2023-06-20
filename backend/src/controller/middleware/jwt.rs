use axum::extract::TypedHeader;
use axum::headers::{authorization::Bearer, Authorization};
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use keycloak::types::UserRepresentation;
use keycloak::{KeycloakAdmin, KeycloakAdminToken};
use serde::{Deserialize, Serialize};

pub(crate) async fn auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let user_name = token_is_valid(auth.token()).await.unwrap();
    if user_name.is_some() {
        request.extensions_mut().insert(user_name.unwrap());
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn token_is_valid(token: &str) -> color_eyre::Result<Option<String>> {
    let url = "http://localhost:8080";
    let user = "admin";
    let password = "password1";
    let client = reqwest::Client::new();
    let admin_token = KeycloakAdminToken::acquire(url, user, password, &client).await?;

    let admin = KeycloakAdmin::new(&url, admin_token, client);

    let cert_begin = b"-----BEGIN PUBLIC KEY-----\n";
    let cert_end = b"\n-----END PUBLIC KEY-----";
    let pub_key = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAyspam31BXJvmFB6R5J/ke8QKu+rHv1/7qh0ee77VTWuQytB6+0KykDFP8skBOyj5NYKdtEYs/+cFG38M9iD805vCQmg9EC6ReVxZ4A3r3jdG5aaHPMy6CwstQGGGd1GHVvJpseyFmN1Wa6miEK/LILNAYcEjULBnmy5SjEP63bgR8qk1vssEpnOPnIkI+ITDbHvY/DzLH2qB3TkyLNkucay7j7pOn21Kg4vVaSs9Dt19WolTXZDGY5LtxNpIbDrja1t1B4mywz6K3io2dDUYuVWtf1ZqPU5EdmSLTM75oX4vTp4QLAWcJHGdOWQDFgHwpFDpJ8/6Zej2gC9AN081XQIDAQAB";
    let mut concat = Vec::<u8>::new();
    concat.extend(cert_begin);
    concat.extend(pub_key.as_bytes());
    concat.extend(cert_end);
    let token_message = decode::<Claims>(
        token,
        &DecodingKey::from_rsa_pem(&concat)?,
        &Validation::new(Algorithm::RS256),
    );

    let user_id = token_message?.claims.sub;

    let user: UserRepresentation = admin
        .realm_users_with_id_get("whatsup", user_id.as_str())
        .await?;

    Ok(user.username)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
    jti: String,
    iss: String,
    sub: String,
    typ: String,
    session_state: String,
    sid: String,
}
