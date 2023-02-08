use sea_orm_migration::prelude::*;

use entity::{refresh_token, user};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RefreshToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RefreshToken::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RefreshToken::Token)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx-refresh_token-uuid")
                            .col(refresh_token::Column::Token),
                    )
                    .col(
                        ColumnDef::new(RefreshToken::UserUuid)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refresh_token-user_uuid")
                            .from(
                                refresh_token::Entity,
                                refresh_token::Column::UserUuid,
                            )
                            .to(user::Entity, user::Column::Uuid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RefreshToken::ExpiryDate)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefreshToken::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum RefreshToken {
    Table,
    Id,
    Token,
    UserUuid,
    ExpiryDate,
}
