use actix::prelude::*;
use actix_interop::FutureInterop;
use log::debug;
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
        debug!("Extracting from feed: {}", msg.0);
        async move {
            let content = reqwest::get(msg.0).await?.bytes().await?;
            let channel = Channel::read_from(&content[..])?;
            let items = channel.items();

            Ok(items.to_vec())
        }
        .interop_actor_boxed(self)
    }
}

#[cfg(test)]
mod tests {
    use actix::Actor;

    use super::Extractor;

    #[actix::test]
    async fn extract_feed() {
        env_logger::init();

        let ext = Extractor::new().start();
        // working rss feed
        {
            let res = ext
                .send(super::ExtractFeedRequest(
                    "http://feeds.bbci.co.uk/news/rss.xml".to_owned(),
                ))
                .await;
            assert_eq!(res.is_ok(), true);
            let res = res.unwrap();
            assert_eq!(res.is_ok(), true);
            let res = res.unwrap();
            assert_eq!(res.len() > 0, true);
        }
        // non-existent rss feed
        {
            let res = ext
                .send(super::ExtractFeedRequest(
                    "http://feeds.example.com/rss.xml".to_owned(),
                ))
                .await;
            assert_eq!(res.is_ok(), true);
            let res = res.unwrap();
            assert_eq!(res.is_err(), true);
        }
    }
}
