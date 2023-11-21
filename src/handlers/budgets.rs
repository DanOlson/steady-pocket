use crate::{
    prelude::*,
    service,
    repository::Repository,
    models::{GetBudgetsDTO, CreateBudgetDTO, UpdateBudgetDTO}
};
use actix_web::{web, get, post, patch, HttpResponse};

#[get("/budgets")]
pub async fn get_budgets(repo: web::Data<dyn Repository>) -> Result<HttpResponse> {
    let budgets = repo.into_inner().budgets().await?;
    let body = GetBudgetsDTO { budgets };

    Ok(HttpResponse::Ok().json(body))
}

#[get("/budgets/{budget_id}")]
pub async fn get_budget(
    repo: web::Data<dyn Repository>,
    budget_id: web::Path<i32>,
) -> Result<HttpResponse> {
    let budget_id = budget_id.into_inner();
    let repo = repo.into_inner();
    let budget_response = service::budget::get_budget(&*repo, budget_id).await?;

    Ok(HttpResponse::Ok().json(budget_response))
}

#[post("/budgets")]
pub async fn create_budget(
    repo: web::Data<dyn Repository>,
    budget: web::Json<CreateBudgetDTO>
) -> Result<HttpResponse> {
    let budget = budget.into_inner().budget;
    let repo = repo.into_inner();
    let budget = service::budget::create_budget(&*repo, budget).await?;

    let response = HttpResponse::Created()
        .insert_header(("Location", format!("/v1/budgets/{}", budget.id)))
        .json(budget);
    Ok(response)
}

#[patch("/budgets/{budget_id}")]
pub async fn update_budget(
    repo: web::Data<dyn Repository>,
    budget: web::Json<UpdateBudgetDTO>,
    budget_id: web::Path<i32>
) -> Result<HttpResponse> {
    let budget = budget.into_inner().budget;
    let repo = repo.into_inner();
    let budget_id = budget_id.into_inner();
    service::budget::update_budget(&*repo, budget_id, budget).await?;

    let response = HttpResponse::NoContent()
        .insert_header(("Location", format!("/api/v1/budgets/{budget_id}")))
        .finish();
    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::test_prelude::*,
        models::{Budget, GetBudgetsDTO}
    };

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

    #[sqlx::test(migrator = "MIGRATOR", fixtures("budget"))]
    async fn test_update_budget(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{"budget":{"name":"Updated Budget Name"}}"#.as_bytes();
        let req = test::TestRequest::patch()
            .uri("/api/v1/budgets/1")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(body)
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let location = response
            .headers()
            .get("Location")
            .map(|v| v.to_str().unwrap())
            .unwrap();
        assert_eq!(location, "/api/v1/budgets/1");
    }

    #[sqlx::test(migrator = "MIGRATOR", fixtures("budget"))]
    async fn test_get_budgets(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/budgets")
            .to_request();
        let response: GetBudgetsDTO = test::call_and_read_body_json(&app, req).await;
        assert_eq!(response.budgets.len(), 1);
    }
}
