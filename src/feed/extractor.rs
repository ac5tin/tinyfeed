use actix::prelude::*;
use actix_interop::{with_ctx, FutureInterop};
use rss::{Channel, Item};

#[derive(Message, Clone)]
#[rtype(result = "Result<Vec<Item>,anyhow::Error>")]
pub struct ExtractFeedRequest(pub String);

// Feed extractor
pub struct Extractor {}

impl Extractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Actor for Extractor {
    type Context = Context<Self>;
}

impl Handler<ExtractFeedRequest> for Extractor {
    type Result = ResponseActFuture<Self, Result<Vec<Item>, anyhow::Error>>;

    fn handle(&mut self, msg: ExtractFeedRequest, _: &mut Self::Context) -> Self::Result {
        async move {
            let content = reqwest::get(msg.0).await?.bytes().await?;
            let channel = Channel::read_from(&content[..])?;
            let items = channel.items();

            Ok(items.to_vec())
        }
        .interop_actor_boxed(self)
    }
}
