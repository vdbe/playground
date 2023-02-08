use entity::user;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Uuid)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx-user-uuid")
                            .col(user::Column::Uuid),
                    )
                    .col(
                        ColumnDef::new(User::Displayname)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx-user-displayname")
                            .col(user::Column::Displayname),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Password).string())
                    .col(ColumnDef::new(User::LastLogin).timestamp())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    Uuid,
    Displayname,
    Email,
    Password,
    LastLogin,
    CreatedAt,
    UpdatedAt,
}
