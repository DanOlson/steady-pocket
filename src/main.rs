mod prelude;
mod config;
mod models;
mod db;
mod errors;
mod handlers;
mod service;
mod repository;

use crate::config::AppConfig;
use crate::prelude::*;
use std::sync::Arc;
use db::Db;
use ::config::Config;
use actix_web::{
    web::{Data, scope},
    App,
    HttpServer
};
use dotenv::dotenv;
use handlers::{
    get_budgets,
    get_budget
};
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
        let repo_arc: Arc<dyn Repository> = Arc::new(repo.clone());
        let repo: Data<dyn Repository> = Data::from(repo_arc);
        App::new()
            .app_data(repo.clone())
            .service(
                scope("/api/v1")
                    .service(get_budgets)
                    .service(get_budget)
            )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);
    server.await?;
    Ok(())
}
