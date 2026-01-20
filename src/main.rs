use actix_web::{App, HttpServer};

mod route_config;
use crate::route_config::route_config;


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