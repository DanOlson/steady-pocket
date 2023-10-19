use std::sync::Arc;
use actix_web::web::{scope, Data, ServiceConfig};
use crate::repository::Repository;

mod budgets;
pub use budgets::*;

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
        );
    }
}

#[cfg(test)]
mod tests {
    use super::api_config;
    use crate::{
        repository::DatabaseRepository,
        db::Db,
        models::Budget
    };
    use actix_web::{App, test, web::ServiceConfig};

    // Setup the API config with an in-memory database
    async fn test_config() -> impl FnOnce(&mut ServiceConfig) {
        let db = Db::connect("sqlite://:memory:").await.unwrap();
        let repo = DatabaseRepository::new(db);
        api_config(repo)
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
        assert_eq!(resp.name, String::from("Simpson Family Budget"));
        assert_eq!(resp.interval_name, String::from("monthly"));
    }
}
