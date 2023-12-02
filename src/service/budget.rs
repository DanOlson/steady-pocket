use crate::{
    prelude::*,
    repository::Repository,
    util::time_util,
    models::{
        BudgetResponse,
        BudgetDTO,
        Budget,
        CreateBudget,
        UpdateBudget
    }
};

pub async fn get_budget(repo: &dyn Repository, budget_id: i32) -> Result<BudgetResponse> {
    let budget = repo.budget(budget_id).await?;
    let since = time_util::start_of_current_month();
    let categories = repo.expense_categories(budget_id, since).await?;
    let category_ids = categories
        .iter()
        .map(|cat| cat.id)
        .collect::<Vec<i32>>();
    let since = time_util::start_of_current_month();
    let expenditures = repo.expenditures(&category_ids, since).await?;
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

pub async fn create_budget(repo: &dyn Repository, budget: CreateBudget) -> Result<Budget> {
    repo.create_budget(budget).await
}

pub async fn update_budget(repo: &dyn Repository, budget_id: i32, budget: UpdateBudget) -> Result<()> {
    repo.update_budget(budget_id, budget).await
}

#[cfg(test)]
mod tests {
    use crate::{
        repository::{DatabaseRepository, Repository},
        db::Db
    };
    use super::{
        UpdateBudget,
        CreateBudget,
        update_budget
    };

    #[tokio::test]
    async fn test_update_budget() {
        let db = Db::connect("sqlite://:memory:").await.unwrap();
        let repo = DatabaseRepository::new(db);
        let budget = CreateBudget {
            name: "Bud".to_string(),
            interval_name: "monthly".to_string()
        };
        let budget = repo.create_budget(budget).await.unwrap();
        let budget_id = budget.id;
        update_budget(
            &repo,
            budget_id,
            UpdateBudget { name: "Budget".to_string() }
        ).await.unwrap();
        let updated_budget = repo.budget(budget_id).await.unwrap();
        assert_eq!(updated_budget.name, "Budget".to_string());
    }
}
