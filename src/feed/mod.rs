use std::time::Duration;

use crate::entity::{feed, post};
use actix::prelude::*;
use actix_interop::{with_ctx, FutureInterop};
use rss::Item;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use self::extractor::ExtractFeedRequest;
mod extractor;

const REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 15); // 15 minutes

#[derive(Message, Clone)]
#[rtype(result = "Result<i32,anyhow::Error>")]
pub struct CreateFeedRequest {
    pub url: String,
    pub title: String,
}

#[derive(Message, Clone)]
#[rtype(result = "Result<(),anyhow::Error>")]
pub struct RefreshFeedRequest();

#[derive(Message, Clone)]
#[rtype(result = "Result<(),anyhow::Error>")]
pub struct FeedItem {
    pub item: Item,
    pub feed_id: i32,
}

pub struct Feed {
    conn: DatabaseConnection,
    extractor: Addr<extractor::Extractor>,
}

impl Feed {
    pub fn new(conn: DatabaseConnection) -> Self {
        let extractor = extractor::Extractor::new().start();
        Self { conn, extractor }
    }
}

impl Actor for Feed {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // loop monitor
        ctx.run_interval(REFRESH_INTERVAL, |_act, ctx| {
            ctx.address().do_send(RefreshFeedRequest());
        });
    }
}

impl Handler<CreateFeedRequest> for Feed {
    type Result = ResponseActFuture<Self, Result<i32, anyhow::Error>>;

    fn handle(&mut self, msg: CreateFeedRequest, _: &mut Self::Context) -> Self::Result {
        async move {
            let conn = with_ctx(|actor: &mut Self, _| actor.conn.clone());
            // check if record already exist
            {
                let record = feed::Entity::find()
                    .filter(feed::Column::Url.eq(msg.url.clone()))
                    .one(&conn)
                    .await?;

                if let Some(rec) = record {
                    return Ok(rec.id);
                }
            }
            // never seen this feed url before, creating new feed record
            let record = feed::ActiveModel {
                title: Set(msg.title),
                url: Set(msg.url),
                ..Default::default()
            };
            let res = feed::Entity::insert(record).exec(&conn).await?;
            // refresh feed
            {
                let addr = with_ctx(|_: &mut Self, ctx: &mut Self::Context| ctx.address());
                addr.do_send(RefreshFeedRequest());
            }

            Ok(res.last_insert_id)
        }
        .interop_actor_boxed(self)
    }
}

impl Handler<RefreshFeedRequest> for Feed {
    type Result = ResponseActFuture<Self, Result<(), anyhow::Error>>;

    fn handle(&mut self, _: RefreshFeedRequest, _: &mut Self::Context) -> Self::Result {
        async move {
            let conn = with_ctx(|actor: &mut Self, _| actor.conn.clone());
            let feeds = feed::Entity::find()
                .filter(feed::Column::Url.is_not_null())
                .all(&conn)
                .await?;

            let extractor = with_ctx(|actor: &mut Self, _| actor.extractor.clone());
            let addr = with_ctx(|_: &mut Self, ctx: &mut Self::Context| ctx.address().clone());
            for feed in feeds {
                if let Ok(Ok(feed_items)) = extractor.send(ExtractFeedRequest(feed.url)).await {
                    for item in feed_items {
                        addr.do_send(FeedItem {
                            item,
                            feed_id: feed.id,
                        });
                    }
                };
            }

            Ok(())
        }
        .interop_actor_boxed(self)
    }
}

impl Handler<FeedItem> for Feed {
    type Result = ResponseActFuture<Self, Result<(), anyhow::Error>>;

    fn handle(&mut self, msg: FeedItem, _: &mut Self::Context) -> Self::Result {
        // insert new feed item
        async move {
            let conn = with_ctx(|actor: &mut Self, _| actor.conn.clone());
            let rec = post::ActiveModel {
                feed_id: Set(msg.feed_id),
                title: Set(msg.item.title().unwrap_or("").to_owned()),
                url: Set(msg.item.link().unwrap_or("").to_owned()),
                author: Set(msg.item.author().unwrap_or("").to_owned()),
                timestamp: Set(msg.item.pub_date().unwrap_or("").to_owned()),
                content: Set(msg.item.content().unwrap_or("").to_owned()),
                ..Default::default()
            };
            rec.insert(&conn).await?;
            Ok(())
        }
        .interop_actor_boxed(self)
    }
}
