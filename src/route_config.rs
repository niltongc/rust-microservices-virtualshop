
use actix_web::web;

use crate::handlers::product::{hello, manual_hello};

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    );
}