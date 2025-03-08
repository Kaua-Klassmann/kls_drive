use sea_orm_migration::prelude::*;

use crate::m20250304_131008_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Document::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Document::Id)
                            .unsigned()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Document::Name).string().not_null())
                    .col(ColumnDef::new(Document::Type).string().not_null())
                    .col(
                        ColumnDef::new(Document::CreatedAt)
                            .date()
                            .not_null()
                            .default(Expr::current_date()),
                    )
                    .col(ColumnDef::new(Document::IdUser).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-document-user-id")
                            .from(Document::Table, Document::IdUser)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Document::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Document {
    Table,
    Id,
    Name,
    Type,
    IdUser,
    CreatedAt,
}
