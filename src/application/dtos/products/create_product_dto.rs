use sea_orm::prelude::Decimal;

#[derive(serde::Deserialize)]
pub struct CreateProductDto{

    pub name: String,
    pub price: Decimal,
    pub description: String,
    pub stock: i64,
    pub image_url: String,
    pub category_id: i32,
}