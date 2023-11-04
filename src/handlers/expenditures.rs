use crate::{
    prelude::*,
    service,
    repository::Repository,
    models::CreateExpenditureDTO
};
use actix_web::{web, post, HttpResponse};

#[post("/expenditures")]
pub async fn create_expenditure(
    repo: web::Data<dyn Repository>,
    create_expenditure: web::Json<CreateExpenditureDTO>
) -> Result<HttpResponse> {
    let repo = repo.into_inner();
    let create_expenditure = create_expenditure.into_inner();
    let expenditure = service::expenditure::create(
        &*repo,
        create_expenditure.expenditure
    ).await?;
    let response = HttpResponse::Created()
        .insert_header(("Location", format!("/api/v1/expenditures/{}", expenditure.id)))
        .json(expenditure);
    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::test_prelude::*,
        models::{CreateBudget, CreateExpenseCategory, Expenditure}
    };

    #[actix_web::test]
    async fn test_create_expenditure() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            db.create_expense_category(CreateExpenseCategory {
                name: "Groceries".to_string(),
                amount: 50000,
                budget_id: budget.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{
            "expenditure": {
                "amount": 12500,
                "vendor": "Kroger",
                "description": "groceries",
                "expense_category_id": 1
            }
        }"#.as_bytes();
        let req = test::TestRequest::post()
            .uri("/api/v1/expenditures")
            .set_payload(body)
            .insert_header(("Content-Type", "application/json"))
            .to_request();
        let expenditure: Expenditure = test::call_and_read_body_json(&app, req).await;
        assert!(expenditure.id > 0);
        assert_eq!(expenditure.amount, 12500);
        assert_eq!(expenditure.vendor, "Kroger".to_string());
        assert_eq!(expenditure.description, "groceries".to_string());
        assert_eq!(expenditure.category_id, 1);
    }

    #[actix_web::test]
    async fn test_create_expenditure_bad_req() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            db.create_expense_category(CreateExpenseCategory {
                name: "Groceries".to_string(),
                amount: 50000,
                budget_id: budget.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{
            "expenditure": {
                "vendor": "Kroger",
                "description": "groceries",
                "expense_category_id": 1
            }
        }"#.as_bytes();
        let req = test::TestRequest::post()
            .uri("/api/v1/expenditures")
            .set_payload(body)
            .insert_header(("Content-Type", "application/json"))
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
