use std::sync::Arc;
use actix_web::web::{scope, Data, ServiceConfig};
use crate::repository::Repository;

mod budgets;
mod expense_categories;
pub use budgets::*;
pub use expense_categories::*;

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
                .service(create_category)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::api_config;
    use crate::{
        prelude::*,
        repository::{DatabaseRepository, Repository},
        db::Db,
        models::{Budget, ExpenseCategory, CreateBudget}
    };
    use std::future::Future;
    use actix_web::{App, test, web::ServiceConfig};

    // Setup the API config with an in-memory database
    async fn test_config() -> impl FnOnce(&mut ServiceConfig) {
        let repo = init_repo().await.unwrap();
        api_config(repo)
    }

    async fn test_config_with_setup<F, Fut>(f: F) -> impl FnOnce(&mut ServiceConfig)
        where
            F: FnOnce(DatabaseRepository) -> Fut,
            Fut: Future<Output = Result<DatabaseRepository>>
    {
        let repo = init_repo().await.unwrap();
        let repo = f(repo).await.unwrap();
        api_config(repo)
    }

    async fn init_repo() -> Result<DatabaseRepository> {
        let db = Db::connect("sqlite://:memory:").await?;
        Ok(DatabaseRepository::new(db))
    }

    #[actix_web::test]
    async fn test_create_budget() {
        let app = test::init_service(
            App::new().configure(test_config().await)
        )
        .await;
        let body = r#"{
            "budget": {
                "name": "Simpson Family Budget",
                "interval_name": "monthly"
            }
        }"#.as_bytes();
        let req = test::TestRequest::post()
            .uri("/api/v1/budgets")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(body)
            .to_request();
        let resp: Budget = test::call_and_read_body_json(&app, req).await;

        assert!(resp.id > 0);
        assert_eq!(resp.name, "Simpson Family Budget".to_string());
        assert_eq!(resp.interval_name, "monthly".to_string());
    }

    #[actix_web::test]
    async fn test_create_category() {
        let config = test_config_with_setup(|db| async {
            db.create_budget(CreateBudget {
                name: "Test".to_string(),
                interval_name: "monthly".to_string()
            }).await?;
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        )
        .await;
        let body = r#"{
            "category": {
                "name": "Housing",
                "amount": 200000,
                "budget_id": 1
            }
        }"#.as_bytes();
        let req = test::TestRequest::post()
            .uri("/api/v1/expense_categories")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(body)
            .to_request();
        // let resp = test::call_service(&app, req).await;
        // assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
        let resp: ExpenseCategory = test::call_and_read_body_json(&app, req).await;
        assert!(resp.id > 0);
        assert_eq!(resp.name, "Housing".to_string());
        assert_eq!(resp.amount, 200000);
        assert_eq!(resp.total_spend_to_date, 0);
    }
}
