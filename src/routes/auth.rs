use actix_web::web;
use crate::handlers::auth::{login, callback};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login", web::get().to(login))
       .route("/auth/callback", web::get().to(callback));
}