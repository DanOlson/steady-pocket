use crate::{
    prelude::*,
    repository::Repository,
    models::{
        ExpenseCategory,
        CreateExpenseCategory,
        UpdateExpenseCategory
    }
};

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
