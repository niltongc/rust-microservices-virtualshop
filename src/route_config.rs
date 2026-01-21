
use actix_web::web;

use crate::handlers::product::{hello, manual_hello, get_product_by_id};

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(hello)
            .service(get_product_by_id)
            .route("/hey", web::get().to(manual_hello))
    );
}