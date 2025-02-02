use actix_web::{web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use std::env;

mod handlers;
mod routes;
mod config;
mod utils;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config::db::create_pool(&database_url)))
            .configure(routes::auth::config)
            .route("/", web::get().to(index)) // Add this line to handle the root path
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}