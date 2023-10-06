mod config;
mod models;
mod db;
mod errors;
mod handlers;
mod service;

use crate::config::ExampleConfig;
use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::{get_budgets, get_budget};
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v1")
                    .service(get_budgets)
                    .service(get_budget)
            )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);
    server.await
}
