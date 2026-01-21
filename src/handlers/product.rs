use actix_web::{HttpResponse, Responder, get, web};

use sea_orm::EntityTrait;
use sea_orm::prelude::Decimal;
use serde::Serialize;

use crate::entity::{prelude::*, products};

use crate::AppState;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/product/{id}")]
pub async fn get_product_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match Products::find_by_id(id).one(&data.db).await {
        Ok(Some(product)) => {
            let dto = ProductDto::from(product);
            HttpResponse::Ok().json(dto)
        }
        Ok(None) => HttpResponse::NotFound().body("Product not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

impl From<products::Model> for ProductDto {
    fn from(model: products::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            price: model.price,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
}