use crate::entity::feed;
use actix::prelude::*;
use actix_interop::{with_ctx, FutureInterop};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

#[derive(Message, Clone)]
#[rtype(result = "Result<i32,anyhow::Error>")]
pub struct CreateFeedRequest {
    pub url: String,
    pub title: String,
}

pub struct Feed {
    conn: DatabaseConnection,
}

impl Feed {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

impl Actor for Feed {
    type Context = Context<Self>;
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

            Ok(res.last_insert_id)
        }
        .interop_actor_boxed(self)
    }
}
