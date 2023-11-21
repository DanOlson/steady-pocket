use std::sync::Arc;
use actix_web::web::{scope, Data, ServiceConfig};
use actix_files as fs;
use crate::repository::Repository;

mod budgets;
mod expense_categories;
mod expenditures;
pub use budgets::*;
pub use expense_categories::*;
pub use expenditures::*;

pub fn api_config(repo: impl Repository + 'static) -> impl FnOnce(&mut ServiceConfig) {
    let repo_arc: Arc<dyn Repository> = Arc::new(repo);
    let repo_data: Data<dyn Repository> = Data::from(repo_arc);
    |cfg: &mut ServiceConfig| {
        cfg.app_data(repo_data)
        .service(
    scope("/api/v1")
                .service(get_budgets)
                .service(get_budget)
                .service(create_budget)
                .service(update_budget)
                .service(get_category)
                .service(create_category)
                .service(update_category)
                .service(delete_category)
                .service(create_expenditure)
                .service(update_expenditure)
                .service(delete_expenditure)
                .service(get_expenditures)
                .service(get_expenditure)
        )
        .service(fs::Files::new("/", "./client/build"));
    }
}

#[cfg(test)]
pub mod test_prelude {
    pub use super::api_config;
    pub use crate::{
        prelude::*,
        repository::{DatabaseRepository, Repository},
        db::{Db, MIGRATOR},
    };
    pub use actix_web::{App, test, http::StatusCode, web::ServiceConfig};
    pub use sqlx::SqlitePool;

    pub async fn test_config_with_pool(pool: sqlx::SqlitePool) -> impl FnOnce(&mut ServiceConfig) {
        let db = Db::new(pool);
        let repo = DatabaseRepository::new(db);
        api_config(repo)
    }

    // Setup the API config with an in-memory database
    pub async fn test_config() -> impl FnOnce(&mut ServiceConfig) {
        let repo = init_repo().await.unwrap();
        api_config(repo)
    }

    pub async fn init_repo() -> Result<DatabaseRepository> {
        let db = Db::connect("sqlite://:memory:").await?;
        Ok(DatabaseRepository::new(db))
    }
}
