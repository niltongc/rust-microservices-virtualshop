use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

use crate::AppState;
use crate::application::services::product_services::{ProductService};
use crate::application::dtos::products::*;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/product/{id}")]
pub async fn get_product_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match ProductService::get_by_id(&data.db, id).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().body("Product not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/product")]
pub async fn get_all_product(data: web::Data<AppState>) -> impl Responder{
    match ProductService::get_all_product(&data.db).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/product")]
pub async fn create_product(
    data: web::Data<AppState>,
    product_data: Json<CreateProductDto>
) -> impl Responder {
    let product_data = product_data.into_inner();

    match ProductService::create_product(&data.db, product_data).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/product/{id}")]
pub async fn update_product(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    product_data: Json<UpdateProductDto>,
) -> impl Responder {
    let id = path.into_inner();

    match ProductService::update_product(&data.db, id, product_data.into_inner()).await {
        Ok(updated_product) => HttpResponse::Ok().json(updated_product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

}

#[delete("/product/{id}")]
pub async fn delete_product(
    data: web::Data<AppState>,
    path: web::Path<i32>
) -> impl Responder{
    let id = path.into_inner();

    match ProductService::delete_product(&data.db,id).await{
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().body("Product not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


