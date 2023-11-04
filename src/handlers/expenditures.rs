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
