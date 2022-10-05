mod app;
mod db;
mod feed;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    // create databse connection
    let conn = db::conn::get_conn().await?;

    // start app
    app::start(conn).await?;
    Ok(())
}
