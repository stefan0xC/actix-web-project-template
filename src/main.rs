use actix_web::{middleware, web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Person {
    id: u32,
    name: String,
}

async fn person(person: web::Path<Person>) -> Result<String> {
    Ok(format!("Welcome {} {}!", person.name, person.id))
}

async fn index() -> &'static str {
    "Hello world!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .route("/people/{id}/{name}", web::get().to(person),)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
