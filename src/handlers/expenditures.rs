use crate::{
    prelude::*,
    service,
    repository::Repository,
    models::{CreateExpenditureDTO, UpdateExpenditureDTO}
};
use serde::Deserialize;
use actix_web::{delete, get, patch, post, web, HttpResponse};

#[derive(Deserialize)]
pub struct QueryString {
    expense_category_id: i32
}

#[get("/expenditures")]
pub async fn get_expenditures(
    repo: web::Data<dyn Repository>,
    query: web::Query<QueryString>,
) -> Result<HttpResponse> {
    let repo = repo.into_inner();
    let query = query.into_inner();

    let expenditures = service::expenditure::for_category(&*repo, query.expense_category_id).await?;
    let response = HttpResponse::Ok()
        .json(expenditures);
    Ok(response)
}


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

#[patch("/expenditures/{id}")]
pub async fn update_expenditure(
    repo: web::Data<dyn Repository>,
    id: web::Path<i32>,
    update: web::Json<UpdateExpenditureDTO>
) -> Result<HttpResponse> {
    let repo = repo.into_inner();
    let id = id.into_inner();
    let update = update.into_inner();
    service::expenditure::update(&*repo, id, update.expenditure).await?;

    let response = HttpResponse::NoContent()
        .insert_header(("Location", format!("/api/v1/expenditures/{id}")))
        .finish();
    Ok(response)
}

#[delete("/expenditures/{id}")]
pub async fn delete_expenditure(
    repo: web::Data<dyn Repository>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let repo = repo.into_inner();
    let id = id.into_inner();

    service::expenditure::delete(&*repo, id).await?;

    let response = HttpResponse::NoContent().finish();
    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::test_prelude::*,
        models::{
            CreateBudget,
            CreateExpenseCategory,
            Expenditure,
            CreateExpenditure
        }
    };

    #[actix_web::test]
    async fn test_get_expenditures() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            let category = db.create_expense_category(CreateExpenseCategory {
                name: "Groceries".to_string(),
                amount: 50000,
                budget_id: budget.id
            }).await.unwrap();
            db.create_expenditure(CreateExpenditure {
                description: "waffles".to_string(),
                vendor: "Waffle House".to_string(),
                amount: 1200,
                expense_category_id: category.id
            }).await.unwrap();
            db.create_expenditure(CreateExpenditure {
                description: "chicken".to_string(),
                vendor: "Chick Fil-A".to_string(),
                amount: 862,
                expense_category_id: category.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/expenditures?expense_category_id=1")
            .insert_header(("Accept", "application/json"))
            .to_request();
        let expenditures: Vec<Expenditure> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(expenditures.len(), 2);
    }

    #[actix_web::test]
    async fn test_get_expenditures_no_filter() {
        let config = test_config().await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/expenditures")
            .insert_header(("Accept", "application/json"))
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

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

    #[actix_web::test]
    async fn test_update_expenditure_amount() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            let category = db.create_expense_category(CreateExpenseCategory {
                name: "Groceries".to_string(),
                amount: 50000,
                budget_id: budget.id
            }).await.unwrap();
            db.create_expenditure(CreateExpenditure {
                description: "waffles".to_string(),
                vendor: "Waffle House".to_string(),
                amount: 1200,
                expense_category_id: category.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{"expenditure":{"amount":1250}}"#.as_bytes();
        let req = test::TestRequest::patch()
            .uri("/api/v1/expenditures/1")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(body)
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let location = response
            .headers()
            .get("Location")
            .map(|l| l.to_str().unwrap())
            .unwrap();
        assert_eq!(location, "/api/v1/expenditures/1");
    }

    #[actix_web::test]
    async fn test_update_expenditure_description() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            let category = db.create_expense_category(CreateExpenseCategory {
                name: "Groceries".to_string(),
                amount: 50000,
                budget_id: budget.id
            }).await.unwrap();
            db.create_expenditure(CreateExpenditure {
                description: "waffles".to_string(),
                vendor: "Waffle House".to_string(),
                amount: 1200,
                expense_category_id: category.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{"expenditure":{"description":"tasty waffles"}}"#.as_bytes();
        let req = test::TestRequest::patch()
            .uri("/api/v1/expenditures/1")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(body)
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let location = response
            .headers()
            .get("Location")
            .map(|l| l.to_str().unwrap())
            .unwrap();
        assert_eq!(location, "/api/v1/expenditures/1");
    }

    #[actix_web::test]
    async fn test_update_expenditure_vendor() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            let category = db.create_expense_category(CreateExpenseCategory {
                name: "Groceries".to_string(),
                amount: 50000,
                budget_id: budget.id
            }).await.unwrap();
            db.create_expenditure(CreateExpenditure {
                description: "waffles".to_string(),
                vendor: "Waffle House".to_string(),
                amount: 1200,
                expense_category_id: category.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{"expenditure":{"vendor":"Denny's"}}"#.as_bytes();
        let req = test::TestRequest::patch()
            .uri("/api/v1/expenditures/1")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(body)
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let location = response
            .headers()
            .get("Location")
            .map(|l| l.to_str().unwrap())
            .unwrap();
        assert_eq!(location, "/api/v1/expenditures/1");
    }

    #[actix_web::test]
    async fn test_delete_expenditure() {
        let config = test_config_with_setup(|db| async {
            let budget = db.create_budget(CreateBudget {
                name: "Test Budget".to_string(),
                interval_name: "monthly".to_string()
            }).await.unwrap();
            let category = db.create_expense_category(CreateExpenseCategory {
                name: "Groceries".to_string(),
                amount: 50000,
                budget_id: budget.id
            }).await.unwrap();
            db.create_expenditure(CreateExpenditure {
                description: "waffles".to_string(),
                vendor: "Waffle House".to_string(),
                amount: 1200,
                expense_category_id: category.id
            }).await.unwrap();
            Ok(db)
        }).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let req = test::TestRequest::delete()
            .uri("/api/v1/expenditures/1")
            .insert_header(("Content-Type", "application/json"))
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
