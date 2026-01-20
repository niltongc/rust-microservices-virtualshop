use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // Category
        manager
            .create_table(
                Table::create()
                    .table("categories")
                    .if_not_exists()
                    .col(pk_auto(Categories::Id))
                    .col(string(Categories::Name))
                    .to_owned(),
            )
            .await?;
        
        // Product
        manager
            .create_table(
                Table::create()
                    .table("products")
                    .if_not_exists()
                    .col(pk_auto(Products::Id))
                    .col(string(Products::Name))
                    .col(decimal(Products::Price))
                    .col(string(Products::Description))
                    .col(big_integer(Products::Stock))
                    .col(string(Products::ImageUrl))
                    .col(integer(Products::CategoryId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_Categories")
                            .from(Products::Table, Products::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
        
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table("categories").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("products").to_owned())
            .await?;

        Ok(())

    }
}

#[derive(Iden)]
enum Products {
    Table,
    Id,
    Name,
    Price,
    Description,
    Stock,
    ImageUrl,
    CategoryId,
}

#[derive(Iden)]
enum Categories {
    Table,
    Id,
    Name,
}

