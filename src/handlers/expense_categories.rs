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
