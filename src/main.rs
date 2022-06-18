mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "budgets")]
    pub struct Budget {
        pub name: String,
    }
}

mod db {
    use crate::{errors::MyError, models::Budget};
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    pub async fn get_budgets(client: &Client) -> Result<Vec<Budget>, MyError> {
        let q = include_str!("../sql/get_budgets.sql");
        let query = client.prepare(q).await.unwrap();

        let res = client.query(&query, &[])
            .await?
            .iter()
            .map(|row| Budget::from_row_ref(row).unwrap())
            .collect::<Vec<Budget>>();

        Ok(res)
    }
}

mod errors {
    use actix_web::{ResponseError, HttpResponse};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

mod handlers {
    use crate::{errors::MyError, db};
    use actix_web::{web, get, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    #[get("/budgets")]
    async fn get_budgets(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
        let budgets = db::get_budgets(&client).await?;

        Ok(HttpResponse::Ok().json(budgets))
    }
}

use crate::config::ExampleConfig;
use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::get_budgets;
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
            )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);
    server.await
}
