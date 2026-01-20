use actix_web::{get, web, HttpResponse, HttpServer, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    );
}