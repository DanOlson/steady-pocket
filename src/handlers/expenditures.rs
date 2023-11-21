use crate::{
    prelude::*,
    service,
    repository::Repository,
    models::{
        GetExpenditureDTO,
        CreateExpenditure,
        UpdateExpenditureDTO,
        ExpendituresQuery
    }
};
use actix_web::{delete, get, patch, post, web, HttpResponse};

#[get("/expenditures")]
pub async fn get_expenditures(
    repo: web::Data<dyn Repository>,
    query: web::Query<ExpendituresQuery>,
) -> Result<HttpResponse> {
    let repo = repo.into_inner();
    let query = query.into_inner();

    let expenditures = service::expenditure::for_query(&*repo, query).await?;
    let response = HttpResponse::Ok().json(expenditures);
    Ok(response)
}

#[get("/expenditures/{id}")]
pub async fn get_expenditure(
    repo: web::Data<dyn Repository>,
    id: web::Path<i32>,
) -> Result<HttpResponse> {
    let repo = repo.into_inner();
    let id = id.into_inner();

    let expenditure = service::expenditure::find(&*repo, id).await?;
    let dto = GetExpenditureDTO { expenditure };
    let response = HttpResponse::Ok().json(dto);
    Ok(response)
}

#[post("/expenditures")]
pub async fn create_expenditure(
    repo: web::Data<dyn Repository>,
    create_expenditure: web::Json<CreateExpenditure>
) -> Result<HttpResponse> {
    let repo = repo.into_inner();
    let create_expenditure = create_expenditure.into_inner();
    let expenditure = service::expenditure::create(
        &*repo,
        create_expenditure
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
        models::{Expenditure, GetExpenditureDTO}
    };

    #[sqlx::test(migrator = "MIGRATOR", fixtures("expenditures"))]
    async fn test_get_expenditures(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/expenditures?expense_category_id=1")
            .insert_header(("Accept", "application/json"))
            .to_request();
        let expenditures: Vec<Expenditure> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(expenditures.len(), 3);
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

    #[sqlx::test(migrator = "MIGRATOR", fixtures("expenditures"))]
    async fn test_get_expenditure(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/expenditures/1")
            .insert_header(("Accept", "application/json"))
            .to_request();
        let expenditure_dto: GetExpenditureDTO = test::call_and_read_body_json(&app, req).await;
        let expenditure = expenditure_dto.expenditure;
        assert_eq!(expenditure.id, 1);
        assert_eq!(expenditure.description, "Waffles".to_string());
        assert_eq!(expenditure.vendor, "Kroger".to_string());
        assert_eq!(expenditure.amount, 1268);
    }

    #[sqlx::test(migrator = "MIGRATOR", fixtures("category"))]
    async fn test_create_expenditure(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
        let app = test::init_service(
            App::new().configure(config)
        ).await;
        let body = r#"{
            "amount": 12500,
            "vendor": "Kroger",
            "description": "groceries",
            "expense_category_id": 1
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
        let config = test_config().await;
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

    #[sqlx::test(migrator = "MIGRATOR", fixtures("expenditures"))]
    async fn test_update_expenditure_amount(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
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

    #[sqlx::test(migrator = "MIGRATOR", fixtures("expenditures"))]
    async fn test_update_expenditure_description(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
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

    #[sqlx::test(migrator = "MIGRATOR", fixtures("expenditures"))]
    async fn test_update_expenditure_vendor(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
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

    #[sqlx::test(migrator = "MIGRATOR", fixtures("expenditures"))]
    async fn test_delete_expenditure(pool: SqlitePool) {
        let config = test_config_with_pool(pool).await;
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
