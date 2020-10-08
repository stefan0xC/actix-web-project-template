use crate::handlers;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(handlers::index))
        .route("/people/{id}/{name}", web::get().to(handlers::person));
}
