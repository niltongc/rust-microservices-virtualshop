use std::env;

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;

use sea_orm::{Database, DatabaseConnection};

use crate::route_config::route_config;

mod route_config;
mod handlers;
mod entity;

struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("before server");

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not found");

    let db = Database::connect(database_url)
        .await
        .expect("DB connection failed");

    let app_state = web::Data::new(AppState {db});
    
    let host = env::var("HOST")
        .expect("HOST not found");

    let port: u16 = env::var("PORT")
        .expect("PORT not found")
        .parse()
        .expect("PORT must be a number");
    println!("running at {}:{}", host, port);

    println!("running at {}:{}",host,port);

        println!("after print");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(route_config)
    })
    .bind((host.to_string(), port))?
    .run()
    .await
}