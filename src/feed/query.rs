use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

use crate::entity::{feed, post};

pub async fn get_feed_list(
    offset: u64,
    limit: u64,
    conn: &DatabaseConnection,
) -> Result<Vec<feed::Model>, anyhow::Error> {
    let feeds = feed::Entity::find()
        .offset(offset)
        .limit(limit)
        .all(conn)
        .await?;

    Ok(feeds)
}
