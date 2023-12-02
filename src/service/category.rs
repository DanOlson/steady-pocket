use crate::{
    prelude::*,
    repository::Repository,
    util::time_util,
    models::{
        GetExpenseCategoryDTO,
        ExpenseCategory,
        CreateExpenseCategory,
        UpdateExpenseCategory
    }
};

pub async fn get_category(repo: &dyn Repository, id: i32) -> Result<GetExpenseCategoryDTO> {
    let mut category = repo.expense_category(id).await?;
    let since = time_util::start_of_current_month();
    let expenditures = repo.expenditures_since(id, since).await?;
    category.expenditure_ids = expenditures.iter().map(|e| e.id).collect();
    category.total_spend_to_date = expenditures.iter().map(|e| e.amount as i64).sum();
    let dto = GetExpenseCategoryDTO {
        category,
        expenditures
    };
    Ok(dto)
}

pub async fn create_category(
    repo: &dyn Repository,
    create_category: CreateExpenseCategory
) -> Result<ExpenseCategory> {
    repo.create_expense_category(create_category).await
}

pub async fn update_category(
    repo: &dyn Repository,
    id: i32,
    update_category: UpdateExpenseCategory
) -> Result<()> {
    repo.update_expense_category(id, update_category).await
}

pub async fn delete(
    repo: &dyn Repository,
    id: i32
) -> Result<()> {
    repo.delete_expense_category(id).await
}
