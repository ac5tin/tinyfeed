use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

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

pub async fn get_feed_posts(
    feed_id: i32,
    offset: u64,
    limit: u64,
    conn: &DatabaseConnection,
) -> Result<Vec<post::Model>, anyhow::Error> {
    let posts = post::Entity::find()
        .filter(post::Column::FeedId.eq(feed_id))
        .order_by_desc(post::Column::Timestamp)
        .offset(offset)
        .limit(limit)
        .all(conn)
        .await?;

    Ok(posts)
}
