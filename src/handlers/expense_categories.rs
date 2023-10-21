use crate::{
    prelude::*,
    service,
    repository::Repository,
    models::CreateExpenseCategoryDTO
};
use actix_web::{web, post, HttpResponse};

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
