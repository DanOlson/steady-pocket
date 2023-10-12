use crate::{prelude::*, service, repository::Repository};
use actix_web::{web, get, HttpResponse};

#[get("/budgets")]
pub async fn get_budgets(repo: web::Data<dyn Repository>) -> Result<HttpResponse> {
    let budgets = repo.into_inner().budgets().await?;

    Ok(HttpResponse::Ok().json(budgets))
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
