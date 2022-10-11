use super::m20220101_000001_create_table::Feed;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::FeedId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-feed_id")
                            .from(Post::Table, Post::FeedId)
                            .to(Feed::Table, Feed::Id),
                    )
                    .to_owned()
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Url).string().not_null().unique_key())
                    .col(ColumnDef::new(Post::Author).string().not_null())
                    .col(
                        ColumnDef::new(Post::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(ColumnDef::new(Post::Content).string().not_null())
                    .col(
                        ColumnDef::new(Post::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Post::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Post::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Post::Table)
                    .name("fk-post-feed_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Post {
    Table,
    Id,
    FeedId,
    Title,
    Url,
    Author,
    Timestamp,
    Content,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
