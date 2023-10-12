use crate::{
    prelude::*,
    repository::Repository,
    models::{
        BudgetResponse,
        BudgetDTO,
    }
};

pub async fn get_budget(repo: &dyn Repository, budget_id: i32) -> Result<BudgetResponse> {
    let budget = repo.budget(budget_id).await?;
    let categories = repo.expense_categories(budget_id).await?;
    let category_ids = categories
        .iter()
        .map(|cat| cat.id)
        .collect::<Vec<i32>>();
    let expenditures = repo.expenditures(&category_ids).await?;
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

