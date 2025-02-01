use oauth2::{basic::BasicClient, AuthUrl, TokenUrl, RedirectUrl, ClientId, ClientSecret};
use std::env;

pub fn create_oauth_client() -> BasicClient {
    let client_id = env::var("OAUTH_CLIENT_ID").expect("OAUTH_CLIENT_ID must be set");
    let client_secret = env::var("OAUTH_CLIENT_SECRET").expect("OAUTH_CLIENT_SECRET must be set");
    let redirect_url = env::var("OAUTH_REDIRECT_URL").expect("OAUTH_REDIRECT_URL must be set");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}