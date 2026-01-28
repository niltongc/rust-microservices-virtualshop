use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::prelude::Decimal;
use serde::Serialize;

use crate::entity::{prelude::*, products};

use crate::AppState;
use crate::services::product_services::ProductService;

#[derive(Debug, Serialize)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
    pub description: String,
}

impl From<products::Model> for ProductDto {
    fn from(model: products::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            price: model.price,
            description: model.description
        }
    }
}

// impl From<Vec<products::Model>> for Vec<ProductDto> {
//     fn from(models: Vec<products::Model>) -> Self {
//         models.into_iter().map(ProductDto::from).collect()
//     }
// }

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[get("/product/{id}")]
// pub async fn get_product_by_id(
//     data: web::Data<AppState>,
//     path: web::Path<i32>,
// ) -> impl Responder {
//     let id = path.into_inner();

//     match Products::find_by_id(id).one(&data.db).await {
//         Ok(Some(product)) => {
//             let dto = ProductDto::from(product);
//             HttpResponse::Ok().json(dto)
//         }
//         Ok(None) => HttpResponse::NotFound().body("Product not found"),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

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

// #[get("/product")]
// pub async fn get_all_product(data: web::Data<AppState>) -> impl Responder{
//     match Products::find().all(&data.db).await {
//         Ok(products) => {
//             let dto: Vec<ProductDto> = products
//                 .into_iter()
//                 .map(ProductDto::from)
//                 .collect();

//             HttpResponse::Ok().json(dto)
//         }
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

#[get("/product")]
pub async fn get_all_product(data: web::Data<AppState>) -> impl Responder{
    match ProductService::get_all_product(&data.db).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Deserialize)]
pub struct CreateUserDto{

    pub name: String,
    pub price: Decimal,
    pub description: String,
    pub stock: i64,
    pub image_url: String,
    pub category_id: i32,
}

#[post("/product")]
pub async fn create_product(
    data: web::Data<AppState>,
    product_data: Json<CreateUserDto>
) -> impl Responder {
    let new_product = products::ActiveModel {
        id: NotSet,
        name: Set(product_data.name.clone()),
        price: Set(product_data.price.clone()),
        description: Set(product_data.description.clone()),
        stock: Set(product_data.stock.clone()),
        image_url: Set(product_data.image_url.clone()),
        category_id: Set(product_data.category_id.clone()),
    };

    match new_product.insert(&data.db).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateProductDto{
    pub name: Option<String>,
    pub price: Option<Decimal>,
    pub description: Option<String>,
    pub stock: Option<i64>,
    pub image_url: Option<String>,
    pub category_id: Option<i32>
}

// #[put("/product/{id}")]
// pub async fn update_product(
//     data: web::Data<AppState>,
//     path: web::Path<i32>,
//     product_data: Json<UpdateProductDto>,
// ) -> impl Responder {
//     let id = path.into_inner();

//     let product = match Products::find_by_id(id).one(&data.db).await {
//         Ok(Some(p)) => p,
//         Ok(None) => return HttpResponse::NotFound().body("Product not found"),
//         Err(_) => return HttpResponse::InternalServerError().finish(),
//     };

//     let mut active_product: products::ActiveModel = product.into();

//     if let Some(name) = &product_data.name {
//         active_product.name = Set(name.clone());
//     }

//     if let Some(price) = &product_data.price {
//         active_product.price = Set(price.clone());
//     }

//     if let Some(description) = &product_data.description {
//         active_product.description = Set(description.clone());
//     }

//     if let Some(stock) = &product_data.stock {
//         active_product.stock = Set(stock.clone());
//     }

//     if let Some(image_url) = &product_data.image_url {
//         active_product.image_url = Set(image_url.clone());
//     }

//     if let Some(category_id) = &product_data.category_id {
//         active_product.category_id = Set(category_id.clone());
//     }

//     match active_product.update(&data.db).await {
//         Ok(updated_product) => HttpResponse::Ok().json(updated_product),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }


// }

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

    match Products::delete_by_id(id).exec(&data.db).await{
        Ok(result) if result.rows_affected == 1 => {
            HttpResponse::NoContent().finish()
        }
        Ok(_) => HttpResponse::NotFound().body("Product not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

    // pub id: i32,
    // pub name: String,
    // pub price: Decimal,
    // pub description: String,
    // pub stock: i64,
    // pub image_url: String,
    // pub category_id: i32,

