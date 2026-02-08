
use actix_web::web;

use crate::handlers::product::*;

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(hello)
            .service(get_product_by_id)
            .service(get_all_product)
            .service(create_product)
            .service(update_product)
            .service(delete_product)
    );
}