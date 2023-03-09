use std::sync::Arc;

use dotenv::dotenv;
use env_logger::Env;
use futures::executor::block_on;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;

use crate::routes::get_routes;

mod db;
mod errors;
mod filters;
mod handlers;
mod jwt;
mod models;
mod requests;
mod routes;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // Logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // DB init
    let db = match block_on(db::run()) {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
    // performing migrations if any
    match Migrator::up(&db, None).await {
        Ok(_) => (),
        Err(e) => log::error!("Error occured during migrations: {e}"),
    }
    log::info!("DB set up");

    // HTTP server
    let db_session: Arc<Mutex<DatabaseConnection>> = Arc::new(Mutex::new(db));
    let routes = get_routes(db_session);

    warp::serve(routes)
        .tls()
        .cert_path("./certs/cert.pem")
        .key_path("./certs/key.pem")
        .run(([127, 0, 0, 1], 8085))
        .await;

    Ok(())
}
