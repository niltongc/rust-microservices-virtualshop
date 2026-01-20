use std::env;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;

use crate::route_config::route_config;

mod route_config;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let host = env::var("HOST")
        .expect("HOST not found");

    let port: u16 = env::var("PORT")
        .expect("PORT not found")
        .parse()
        .expect("PORT must be a number");

    println!("running at {}:{}",host,port);

    HttpServer::new(|| {
        App::new()
            .configure(route_config)
    })
    .bind((host.to_string(), port))?
    .run()
    .await
}