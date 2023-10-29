use crate::prelude::*;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

#[derive(Debug, Clone)]
pub struct Db(SqlitePool);

mod budgets;
mod categories;
mod expenditures;

impl Db {
    pub async fn connect(db_url: &str) -> Result<Self> {
        if !Sqlite::database_exists(db_url).await? {
            Sqlite::create_database(db_url).await?;
        }
        let pool = SqlitePool::connect(db_url).await?;
        sqlx::migrate!("./src/db/migrations").run(&pool).await?;
        Ok(Self(pool))
    }
}
