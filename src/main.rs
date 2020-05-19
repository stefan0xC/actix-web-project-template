mod config;
mod handlers;
mod models;
mod routes;

use actix_web::HttpServer;
use actix_web::App;
use actix_web::middleware;
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    dotenv().ok();
    env_logger::init();

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
