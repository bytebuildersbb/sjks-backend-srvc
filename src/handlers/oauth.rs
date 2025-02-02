use actix_web::{HttpRequest, HttpResponse};
use oauth2::CsrfToken;
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

pub async fn callback(req: HttpRequest) -> HttpResponse {
    let code = req.query_string().split('=').nth(1).unwrap();
    let client = create_oauth_client();
    let token_result = client
        .exchange_code(oauth2::AuthorizationCode::new(code.to_string()))
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    match token_result {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}