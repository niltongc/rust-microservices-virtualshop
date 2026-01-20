use actix_web::{App, HttpServer};

use crate::route_config::route_config;

mod route_config;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("running at localhost:8080");

    HttpServer::new(|| {
        App::new()
            .configure(route_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}