mod config;
mod handlers;
mod models;
mod routes;

#[macro_use]
extern crate log;
use actix_web::middleware;
use actix_web::App;
use actix_web::HttpServer;
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    let config = config::Config::from_env().unwrap();
    let addr: String = config.server.to_string();

    println!("Listening on http://{}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::routes)
    })
    .bind(addr)?
    .run()
    .await
}
