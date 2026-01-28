use sea_orm::EntityTrait;
use sea_orm::{DatabaseConnection};

use crate::{handlers::product::ProductDto};
use crate::entity::{prelude::*};

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

}