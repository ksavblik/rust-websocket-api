use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Book::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Book::Title).string().not_null())
                    .col(ColumnDef::new(Book::Author).string().not_null())
                    .col(ColumnDef::new(Book::Price).string())
                    .col(ColumnDef::new(Book::PublishDate).date_time())
                    .col(ColumnDef::new(Book::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Book::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Book {
    Table,
    Id,
    Title,
    Author,
    PublishDate,
    Price,
    CreatedAt,
    UpdatedAt
}
