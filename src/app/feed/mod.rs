use actix_web::{
    error::ErrorInternalServerError,
    get, post,
    web::{Data, Json, Query},
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

#[derive(Deserialize)]
pub struct GetFeedListQuery {
    limit: u64,
    offset: u64,
}
#[get("")]
pub async fn get_feed_list(
    state: Data<AppState>,
    req: Query<GetFeedListQuery>,
) -> Result<HttpResponse, Error> {
    let feeds = feed::query::get_feed_list(req.offset, req.limit, &state.db)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(json!({"status":"ok","feeds": feeds})))
}
