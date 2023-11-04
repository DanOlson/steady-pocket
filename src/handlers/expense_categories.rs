use crate::{
    prelude::*,
    service,
    repository::Repository,
    models::{CreateExpenseCategoryDTO, UpdateExpenseCategoryDTO}
};
use actix_web::{web, get, patch, post, HttpResponse};

#[post("/expense_categories")]
pub async fn create_category(
    repo: web::Data<dyn Repository>,
    category: web::Json<CreateExpenseCategoryDTO>
) -> Result<HttpResponse> {
    let category = category.into_inner().category;
    let repo = repo.into_inner();
    let category = service::category::create_category(&*repo, category).await?;
    let response = HttpResponse::Created()
        .insert_header(("Location", format!("/api/v1/expense_categories/{}", category.id)))
        .json(category);
    Ok(response)
}

#[patch("/expense_categories/{id}")]
pub async fn update_category(
    repo: web::Data<dyn Repository>,
    category: web::Json<UpdateExpenseCategoryDTO>,
    id: web::Path<i32>
) -> Result<HttpResponse> {
    let repo = repo.into_inner();
    let category = category.into_inner().category;
    let id = id.into_inner();
    service::category::update_category(&*repo, id, category).await?;
    let response = HttpResponse::NoContent()
        .insert_header(("Location", format!("/api/v1/expense_categories/{id}")))
        .finish();
    Ok(response)
}

#[get("/expense_categories/{id}")]
pub async fn get_category(
    repo: web::Data<dyn Repository>,
    id: web::Path<i32>
) -> Result<HttpResponse> {
    let repo = repo.into_inner();
    let id = id.into_inner();
    let category = service::category::get_category(&*repo, id).await?;
    let response = HttpResponse::Ok().json(category);
    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::test_prelude::*,
        models::{
            CreateBudget,
            ExpenseCategory,
            CreateExpenseCategory,
        }
    };

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
        ).await;
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
        let resp: ExpenseCategory = test::call_and_read_body_json(&app, req).await;
        assert!(resp.id > 0);
        assert_eq!(resp.name, "Housing".to_string());
        assert_eq!(resp.amount, 200000);
        assert_eq!(resp.total_spend_to_date, 0);
    }

    #[actix_web::test]
    async fn test_update_category_name() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            db.create_expense_category(CreateExpenseCategory {
                name: "Mortgage".to_string(),
                amount: 200000,
                budget_id: budget.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{"category":{"name":"Mortgage Payment"}}"#.as_bytes();
        let req = test::TestRequest::patch()
            .uri("/api/v1/expense_categories/1")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(body)
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn test_update_category_amount() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            db.create_expense_category(CreateExpenseCategory {
                name: "Mortgage".to_string(),
                amount: 200000,
                budget_id: budget.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{"category":{"amount":250000}}"#.as_bytes();
        let req = test::TestRequest::patch()
            .uri("/api/v1/expense_categories/1")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(body)
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn test_update_category_name_and_amount() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            db.create_expense_category(CreateExpenseCategory {
                name: "Mortgage".to_string(),
                amount: 200000,
                budget_id: budget.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{"category":{"name":"Cabbage","amount":250000}}"#.as_bytes();
        let req = test::TestRequest::patch()
            .uri("/api/v1/expense_categories/1")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(body)
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn test_get_category() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            db.create_expense_category(CreateExpenseCategory {
                name: "Mortgage".to_string(),
                amount: 200000,
                budget_id: budget.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/expense_categories/1")
            .to_request();
        let response: ExpenseCategory = test::call_and_read_body_json(&app, req).await;
        assert_eq!(response.id, 1);
        assert_eq!(response.name, "Mortgage".to_string());
    }
}
