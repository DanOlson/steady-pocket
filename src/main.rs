mod prelude;
mod config;
mod models;
mod db;
mod errors;
mod handlers;
mod service;
mod repository;
mod util;

use crate::config::AppConfig;
use crate::prelude::*;
use db::Db;
use ::config::Config;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use handlers::*;
use repository::*;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();
    let config: AppConfig = config_.try_deserialize().unwrap();
    let db = Db::connect(&config.database_url).await?;
    let repo = DatabaseRepository::new(db);
    let server = HttpServer::new(move || {
        // TODO: this should be revisited - cloning the repo
        // in the application factory doesn't seem right.
        App::new().configure(api_config(repo.clone()))
    })
    .bind(&config.server_addr)?
    .run();
    println!("Server running at http://{}/", config.server_addr);
    server.await?;
    Ok(())
}
