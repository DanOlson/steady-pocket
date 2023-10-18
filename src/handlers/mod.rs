mod budgets;
pub use budgets::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::{
        repository::{
            Repository,
            DatabaseRepository
        },
        db::Db,
        models::{CreateBudget, CreateBudgetDTO, Budget}
    };
    use actix_web::{App, test, web::{scope, Data}, http::StatusCode};

    #[actix_web::test]
    async fn test_create_budget() {
        let db = Db::connect("sqlite://:memory:").await.unwrap();
        let repo = DatabaseRepository::new(db);
        let repo_arc: Arc<dyn Repository> = Arc::new(repo.clone());
        let repo_data: Data<dyn Repository> = Data::from(repo_arc);
        let app = test::init_service(
            App::new()
                .app_data(repo_data)
                .service(
                    scope("/api/v1")
                        .service(create_budget)
                )
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
        // let response = test::call_service(&app, req).await;
        // assert_eq!(response.status(), StatusCode::CREATED);
        let resp: Budget = test::call_and_read_body_json(&app, req).await;

        assert!(resp.id > 0);
        assert_eq!(resp.name, String::from("Simpson Family Budget"));
        assert_eq!(resp.interval_name, String::from("monthly"));
    }
}
