use sea_orm::prelude::Decimal;
use serde::Serialize;

use crate::entity::products;

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