use crate::models;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

pub async fn person(person: web::Path<models::Person>) -> impl Responder {
    let id = person.id;
    let name = person.name.clone();
    HttpResponse::Ok().json(models::Person { id, name })
}

pub async fn index() -> &'static str {
    "Hello world!"
}

pub async fn echo(req_body: String) -> impl Responder {
    info!("{}", req_body.clone());
    HttpResponse::Ok().body(req_body)
}
