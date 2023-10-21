use crate::{
    prelude::*,
    db::Db,
    models::{
        CreateExpenseCategory,
        ExpenseCategory,
    }
};
use sqlx::{sqlite::SqliteRow, Row};

impl Db {
    pub async fn create_category(&self, category: CreateExpenseCategory) -> Result<ExpenseCategory> {
        let q = include_str!("sql/create_expense_category.sql");
        let category = sqlx::query(q)
            .bind(category.name)
            .bind(category.amount)
            .bind(category.budget_id)
            .map(|row: SqliteRow| {
                ExpenseCategory {
                    id: row.get("id"),
                    name: row.get("name"),
                    amount: row.get("amount"),
                    budget_id: row.get("budget_id"),
                    total_spend_to_date: 0,
                    expenditure_ids: vec![]
                }
            })
            .fetch_one(&self.0)
            .await?;

        Ok(category)
    }
}
