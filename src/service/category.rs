use crate::{
    prelude::*,
    repository::Repository,
    models::{CreateExpenseCategory, ExpenseCategory}
};

pub async fn create_category(
    repo: &dyn Repository,
    create_category: CreateExpenseCategory
) -> Result<ExpenseCategory> {
    repo.create_expense_category(create_category).await
}
