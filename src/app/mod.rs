use actix::Actor;
use actix::Addr;
use actix_web::web::Data;
use actix_web::{
    middleware::Logger,
    web::{self, ServiceConfig},
    App, HttpRequest, HttpServer,
};
use log::info;
use sea_orm::DatabaseConnection;

use crate::feed as feeder;

use self::feed::create_feed;
use self::feed::get_feed_list;
mod feed;

pub struct AppState {
    pub db: DatabaseConnection,
    pub feed: Addr<feeder::Feed>,
}

async fn ping(_state: web::Data<AppState>, _req: HttpRequest) -> &'static str {
    "pong"
}

pub async fn start(db: DatabaseConnection) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(AppState {
                db: db.to_owned(),
                feed: feeder::Feed::new(db.to_owned()).start(),
            }))
            .configure(routes)
    });
    info!("Starting server at 0.0.0.0:8080");
    server.bind(("0.0.0.0", 8080))?.run().await?;
    Ok(())
}

fn routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/ping").to(ping));
    cfg.service(
        web::scope("/api").service(
            web::scope("/v1").service(
                web::scope("/feed")
                    .service(create_feed)
                    .service(get_feed_list),
            ),
        ),
    );
}
