
use sea_orm::{DbErr};
use sea_orm::{DatabaseConnection};

use crate::handlers::product::{UpdateProductDto};
use crate::{handlers::product::ProductDto};
use crate::repositories::product_repository::ProductRepository;
use crate::application::dtos::products::create_product_dto::*;
pub struct ProductService;

impl ProductService {
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<ProductDto>, sea_orm::DbErr> {

        let product = ProductRepository::get_by_id(db, id).await?;

        Ok(product.map(ProductDto::from))
    }

    pub async fn get_all_product(
        db: &DatabaseConnection,
    ) -> Result<Vec<ProductDto>, sea_orm::DbErr>{
        let products = ProductRepository::get_all_product(db).await?;

        Ok(products.into_iter().map(ProductDto::from).collect())
    }

    pub async fn create_product(
        db: &DatabaseConnection,
        product_data: CreateProductDto
    ) -> Result<ProductDto, sea_orm::DbErr>{

        let product_created = ProductRepository::create_product(db, product_data).await?;

        Ok(ProductDto::from(product_created))
    }

    pub async fn update_product(
        db: &DatabaseConnection,
        id: i32,
        product_dto: UpdateProductDto,
    ) -> Result<Option<ProductDto>, sea_orm::DbErr>{

        let product = ProductRepository::get_by_id(db, id).await?;

        let product = match product {
            Some(p) => p,
            None => return Ok(None),
        };

        let updated_product = ProductRepository::update_product(db, product_dto, product).await?;

        Ok(Some(ProductDto::from(updated_product)))
    }

    pub async fn delete_product(
        db: &DatabaseConnection,
        id: i32
    ) -> Result<bool, DbErr> {
        ProductRepository::delete_product(db, id).await
    }


}