use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Classes::Table)
                    .col(pk_auto(Classes::Id))
                    .col(string_uniq(Classes::Name))
                    .col(text(Classes::Description))
                    .col(string(Classes::DisplayName))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Classes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Classes {
    Table,
    Id,
    DisplayName,
    Name,
    Description
}


