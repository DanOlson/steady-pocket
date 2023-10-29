use crate::{
    prelude::*,
    db::Db,
    models::{
        CreateExpenseCategory,
        UpdateExpenseCategory,
        ExpenseCategory,
    }
};
use sqlx::{
    sqlite::SqliteRow,
    Row,
    QueryBuilder
};

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

    pub async fn update_expense_category(&self, id: i32, category: UpdateExpenseCategory) -> Result<()> {
        let mut builder = QueryBuilder::new("update expense_categories set ");
        let mut separator = builder.separated(", ");

        if let Some(name) = category.name {
            separator.push("name = ");
            separator.push_bind_unseparated(name);
        }

        if let Some(amount) = category.amount {
            separator.push("amount = ");
            separator.push_bind_unseparated(amount);
        }

        builder.push(" where id = ");
        builder.push_bind(id);

        builder
            .build()
            .execute(&self.0)
            .await?;

        Ok(())
    }

    pub async fn get_category(&self, id: i32) -> Result<ExpenseCategory> {
        let q = include_str!("sql/get_expense_category.sql");
        let category = sqlx::query(q)
            .bind(id)
            .map(|row: SqliteRow| {
                let expenditure_ids: Vec<i32> = row.get::<String, &str>("expenditure_ids")
                    .split(' ')
                    .filter_map(|n| n.parse::<i32>().ok())
                    .collect();

                ExpenseCategory {
                    id: row.get("id"),
                    name: row.get("name"),
                    amount: row.get("amount"),
                    budget_id: row.get("budget_id"),
                    total_spend_to_date: row.get("total_spend_to_date"),
                    expenditure_ids,
                }
            })
            .fetch_one(&self.0)
            .await?;

        Ok(category)
    }

    pub async fn get_categories(&self, budget_id: i32) -> Result<Vec<ExpenseCategory>> {
        let q = include_str!("sql/get_budget_expense_categories.sql");
        let categories = sqlx::query(q)
            .bind(budget_id)
            .map(|row: SqliteRow| {
                let expenditure_ids: Vec<i32> = row.get::<String, &str>("expenditure_ids")
                    .split(' ')
                    .filter_map(|n| n.parse::<i32>().ok())
                    .collect();

                ExpenseCategory {
                    id: row.get("id"),
                    name: row.get("name"),
                    amount: row.get("amount"),
                    budget_id: row.get("budget_id"),
                    total_spend_to_date: row.get("total_spend_to_date"),
                    expenditure_ids,
                }
            })
            .fetch_all(&self.0)
            .await?;

        Ok(categories)
    }
}
