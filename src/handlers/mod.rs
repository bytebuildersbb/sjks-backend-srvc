use actix_web::{web, HttpResponse};
use oauth2::{basic::BasicClient, CsrfToken};
use crate::utils::oauth::create_oauth_client;

pub async fn login() -> HttpResponse {
    let client = create_oauth_client();
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .url();
    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

pub async fn callback(req: web::HttpRequest) -> HttpResponse {
    let code = req.query_string().split('=').nth(1).unwrap();
    let client = create_oauth_client();
    let token_result = client
        .exchange_code(oauth2::AuthorizationCode::new(code.to_string()))
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    match token_result {
        Ok(token) => HttpResponse::Ok().body(format!("Token: {:?}", token)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to exchange code"),
    }
}