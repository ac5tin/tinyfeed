use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{Data, Json},
    Error, HttpResponse,
};
use log::error;
use migration::tests_cfg::json;
use serde::{Deserialize, Serialize};

use crate::{app::AppState, feed};

#[derive(Serialize, Deserialize)]
pub struct CreateFeedReq {
    url: String,
    title: String,
}

#[post("")]
pub async fn create_feed(
    state: Data<AppState>,
    req: Json<CreateFeedReq>,
) -> Result<HttpResponse, Error> {
    let req = req.into_inner();
    let feed_id = match state
        .feed
        .send(feed::CreateFeedRequest {
            title: req.title,
            url: req.url,
        })
        .await
    {
        Ok(r) => match r {
            Ok(feed_id) => feed_id,
            Err(e) => {
                error!("Error creating feed: {}", e);
                return Err(ErrorInternalServerError(e.to_string()));
            }
        },
        Err(err) => {
            error!("Feed actor mailbox error: {}", err);
            return Err(ErrorInternalServerError(err.to_string()));
        }
    };
    Ok(HttpResponse::Ok().json(json!({"status":"ok","feed_id":feed_id})))
}
