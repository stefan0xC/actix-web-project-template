use crate::models;
use actix_web::web;
use actix_web::Responder;

pub async fn person(person: web::Path<models::Person>) -> impl Responder {
    let id = person.id;
    let name = person.name.clone();
    web::HttpResponse::Ok().json(models::Person { id, name })
}

pub async fn index() -> &'static str {
    "Hello world!"
}
