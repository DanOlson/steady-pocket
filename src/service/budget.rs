use crate::{
    errors::MyError,
    db,
    models::{
        BudgetResponse,
        BudgetDTO,
    }
};
use deadpool_postgres::Client;

pub async fn get_budget(client: &Client, budget_id: i32) -> Result<BudgetResponse, MyError> {
    let budget = db::get_budget(client, budget_id).await?;
    let categories = db::get_categories(client, budget_id).await?;
    let category_ids = categories
        .iter()
        .map(|cat| cat.id)
        .collect::<Vec<i32>>();
    let expenditures = db::get_expenditures(client, &category_ids).await?;
    let dto = BudgetDTO {
        id: budget_id,
        name: budget.name,
        category_ids
    };
    Ok(BudgetResponse {
        budget: dto,
        categories,
        expenditures,
    })
}

