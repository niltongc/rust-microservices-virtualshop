use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, get, post, web};

use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::prelude::Decimal;
use serde::Serialize;

use crate::entity::{prelude::*, products};

use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ProductDto {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
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

// impl From<Vec<products::Model>> for Vec<ProductDto> {
//     fn from(models: Vec<products::Model>) -> Self {
//         models.into_iter().map(ProductDto::from).collect()
//     }
// }

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

    match Products::find_by_id(id).one(&data.db).await {
        Ok(Some(product)) => {
            let dto = ProductDto::from(product);
            HttpResponse::Ok().json(dto)
        }
        Ok(None) => HttpResponse::NotFound().body("Product not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/product")]
pub async fn get_all_product(data: web::Data<AppState>) -> impl Responder{
    match Products::find().all(&data.db).await {
        Ok(products) => {
            let dto: Vec<ProductDto> = products
                .into_iter()
                .map(ProductDto::from)
                .collect();

            HttpResponse::Ok().json(dto)
        }
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

    // pub id: i32,
    // pub name: String,
    // #[sea_orm(column_type = "Decimal(Some((10, 0)))")]
    // pub price: Decimal,
    // pub description: String,
    // pub stock: i64,
    // pub image_url: String,
    // pub category_id: i32,

