use crate::{errors::MyError, db};
use crate::service;
use actix_web::{web, get, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

#[get("/budgets")]
pub async fn get_budgets(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let budgets = db::get_budgets(&client).await?;

    Ok(HttpResponse::Ok().json(budgets))
}

#[get("/budgets/{budget_id}")]
pub async fn get_budget(
    db_pool: web::Data<Pool>,
    budget_id: web::Path<i32,>
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let budget_id = budget_id.into_inner();
    let budget_response = service::budget::get_budget(&client, budget_id).await?;

    Ok(HttpResponse::Ok().json(budget_response))
}
