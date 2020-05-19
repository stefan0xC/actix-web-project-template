use actix_web::web;
use crate::handlers;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
       .service(web::resource("/").to(handlers::index))
       .route("/people/{id}/{name}", web::get().to(handlers::person));
}
