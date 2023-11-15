use crate::{
    prelude::*,
    service,
    repository::Repository,
    models::{CreateExpenseCategoryDTO, UpdateExpenseCategoryDTO}
};
use actix_web::{web, delete, get, patch, post, HttpResponse};

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

#[delete("/expense_categories/{id}")]
pub async fn delete_category(
    repo: web::Data<dyn Repository>,
    id: web::Path<i32>
) -> Result<HttpResponse> {
    let repo = repo.into_inner();
    let id = id.into_inner();
    service::category::delete(&*repo, id).await?;

    let response = HttpResponse::NoContent().finish();
    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::{
        handlers::test_prelude::*,
        models::ExpenseCategory
    };

    #[sqlx::test(migrator = "MIGRATOR", fixtures("budget"))]
    async fn test_create_category(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
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

    #[sqlx::test(migrator = "MIGRATOR", fixtures("category"))]
    async fn test_update_category_name(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
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

    #[sqlx::test(migrator = "MIGRATOR", fixtures("category"))]
    async fn test_update_category_amount(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{"category":{"amount":200000}}"#.as_bytes();
        let req = test::TestRequest::patch()
            .uri("/api/v1/expense_categories/1")
            .insert_header(("Content-Type", "application/json"))
            .set_payload(body)
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[sqlx::test(migrator = "MIGRATOR", fixtures("category"))]
    async fn test_update_category_name_and_amount(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
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

    #[sqlx::test(migrator = "MIGRATOR", fixtures("category"))]
    async fn test_get_category(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
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

    #[actix_web::test]
    async fn test_get_category_not_found() {
        let app = test::init_service(
            App::new().configure(test_config().await)
        ).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/expense_categories/1337")
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[sqlx::test(migrator = "MIGRATOR", fixtures("category"))]
    async fn test_delete_category(pool: SqlitePool) {
        let conf = test_config_with_pool(pool).await;
        let app = test::init_service(
            App::new().configure(conf)
        ).await;
        let req = test::TestRequest::delete()
            .uri("/api/v1/expense_categories/1")
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let req = test::TestRequest::get()
            .uri("/api/v1/expense_categories/1")
            .to_request();
        let response = test::call_service(&app, req).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
