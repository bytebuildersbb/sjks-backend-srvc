use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers;
mod routes;
mod config;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config::db::create_pool(&database_url)))
            .configure(routes::auth::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}