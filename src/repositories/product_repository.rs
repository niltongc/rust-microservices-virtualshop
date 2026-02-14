use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};

use crate::entity::products::{self};
use crate::entity::{prelude::*};
use crate::application::dtos::products::*;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<products::Model>, DbErr> {
        Products::find_by_id(id).one(db).await
    }

    pub async fn get_all_product(
        db: &DatabaseConnection,
    ) -> Result<Vec<products::Model>, DbErr> {
        Products::find().all(db).await
    }

    pub async fn create_product(
        db: &DatabaseConnection,
        product_data: CreateProductDto,
    ) -> Result<products::Model, DbErr>{
        let new_product = products::ActiveModel {
            id: NotSet,
            name: Set(product_data.name.clone()),
            price: Set(product_data.price.clone()),
            description: Set(product_data.description.clone()),
            stock: Set(product_data.stock.clone()),
            image_url: Set(product_data.image_url.clone()),
            category_id: Set(product_data.category_id.clone()),
        };

        new_product.insert(db).await
    }

    pub async fn update_product(
        db: &DatabaseConnection,
        product_dto: UpdateProductDto,
        model: products::Model,
    ) -> Result<products::Model, DbErr> {
        let mut active: products::ActiveModel = model.into();

        if let Some(name) = product_dto.name {
            active.name = Set(name);
        }

        if let Some(price) = product_dto.price {
            active.price = Set(price);
        }

        if let Some(description) = product_dto.description {
            active.description = Set(description);
        }

        if let Some(stock) = product_dto.stock {
            active.stock = Set(stock);
        }

        if let Some(image_url) = product_dto.image_url {
            active.image_url = Set(image_url);
        }

        if let Some(category_id) = product_dto.category_id {
            active.category_id = Set(category_id);
        }

        active.update(db).await
    }

    pub async fn delete_product(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool, DbErr> {
        let result = Products::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected == 1)
    }
}