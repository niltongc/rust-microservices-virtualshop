
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{DatabaseConnection};

use crate::handlers::product::{CreateProductDto, UpdateProductDto};
use crate::{handlers::product::ProductDto};
use crate::entity::{prelude::*, products};

pub struct ProductService;

impl ProductService {
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<ProductDto>, sea_orm::DbErr> {

        let product = Products::find_by_id(id).one(db).await?;

        Ok(product.map(ProductDto::from))
    }

    pub async fn get_all_product(
        db: &DatabaseConnection,
    ) -> Result<Vec<ProductDto>, sea_orm::DbErr>{
        let products = Products::find().all(db).await?;

        Ok(products.into_iter().map(ProductDto::from).collect())
    }

    pub async fn create_product(
        db: &DatabaseConnection,
        product_data: CreateProductDto
    ) -> Result<ProductDto, sea_orm::DbErr>{
        let new_product = products::ActiveModel {
            id: NotSet,
            name: Set(product_data.name.clone()),
            price: Set(product_data.price.clone()),
            description: Set(product_data.description.clone()),
            stock: Set(product_data.stock.clone()),
            image_url: Set(product_data.image_url.clone()),
            category_id: Set(product_data.category_id.clone()),
        };

        let product_created = new_product.insert(db).await?;

        Ok(ProductDto::from(product_created))
    }

    pub async fn update_product(
        db: &DatabaseConnection,
        id: i32,
        product_dto: UpdateProductDto,
    ) -> Result<Option<ProductDto>, sea_orm::DbErr>{

        let product = Products::find_by_id(id).one(db).await?;

        let product = match product {
            Some(p) => p,
            None => return Ok(None),
        };

        let mut active_product: products::ActiveModel = product.into();

        if let Some(name) = product_dto.name {
            active_product.name = Set(name);
        }

        if let Some(price) = product_dto.price {
            active_product.price = Set(price);
        }

        if let Some(description) = product_dto.description {
            active_product.description = Set(description);
        }

        if let Some(stock) = product_dto.stock {
            active_product.stock = Set(stock);
        }

        if let Some(image_url) = product_dto.image_url {
            active_product.image_url = Set(image_url);
        }

        if let Some(category_id) = product_dto.category_id {
            active_product.category_id = Set(category_id);
        }

        let updated_product = active_product.update(db).await?;

        Ok(Some(ProductDto::from(updated_product)))
    }


}