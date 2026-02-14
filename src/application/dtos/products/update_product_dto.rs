use sea_orm::prelude::Decimal;

#[derive(serde::Deserialize)]
pub struct UpdateProductDto{
    pub name: Option<String>,
    pub price: Option<Decimal>,
    pub description: Option<String>,
    pub stock: Option<i64>,
    pub image_url: Option<String>,
    pub category_id: Option<i32>
}