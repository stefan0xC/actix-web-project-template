mod config;
mod handlers;
mod models;
mod routes;

use actix_web::HttpServer;
use actix_web::App;
use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let addr: String = config.server.to_string();

    println!("Listening on http://{}", addr);

    HttpServer::new(|| {
        App::new()
            .configure(routes::routes)
    })
    .bind(addr)?
    .run()
    .await
}
