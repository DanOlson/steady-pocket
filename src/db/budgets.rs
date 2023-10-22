use crate::{
    prelude::*,
    db::Db,
    models::{
        Budget,
        CreateBudget,
        UpdateBudget,
        ExpenseCategory,
        Expenditure
    }
};
use sqlx::{sqlite::SqliteRow, Row};

impl Db {
    pub async fn get_budgets(&self) -> Result<Vec<Budget>> {
        let q = include_str!("sql/get_budgets.sql");
        let budgets = sqlx::query(q)
            .map(|row: SqliteRow| {
                Budget {
                    id: row.get("id"),
                    name: row.get("name"),
                    interval_name: row.get("budget_interval")
                }
            })
            .fetch_all(&self.0)
            .await?;

        Ok(budgets)
    }

    pub async fn get_budget(&self, budget_id: i32) -> Result<Budget> {
        let q = include_str!("sql/get_budget.sql");
        let budget = sqlx::query(q)
            .bind(budget_id)
            .map(|row: SqliteRow| {
                Budget {
                    id: row.get("id"),
                    name: row.get("name"),
                    interval_name: row.get("interval_name"),
                }
            })
            .fetch_one(&self.0)
            .await?;

        Ok(budget)
    }

    pub async fn create_budget(&self, budget: CreateBudget) -> Result<Budget> {
        let q = include_str!("sql/create_budget.sql");
        let budget = sqlx::query(q)
            .bind(budget.name)
            .bind(budget.interval_name)
            .map(|row: SqliteRow| {
                Budget {
                    id: row.get::<i32, &str>("id"),
                    name: row.get("name"),
                    interval_name: row.get("budget_interval")
                }
            })
            .fetch_one(&self.0)
            .await?;

        Ok(budget)
    }

    pub async fn update_budget(&self, budget_id: i32, budget: UpdateBudget) -> Result<()> {
        let q = include_str!("sql/update_budget.sql");
        sqlx::query(q)
            .bind(budget.name)
            .bind(budget_id)
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
                    .map(|n| n.parse::<i32>().unwrap())
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

    pub async fn get_expenditure(&self, id: i32) -> Result<Expenditure> {
        let q = include_str!("sql/get_expenditure.sql");
        let expenditure = sqlx::query(q)
            .bind(id)
            .map(|row: SqliteRow| {
                Expenditure {
                    id: row.get("id"),
                    description: row.get("description"),
                    vendor: row.get("vendor"),
                    amount: row.get("amount"),
                    category_id: row.get("category_id"),
                }
            })
            .fetch_one(&self.0)
            .await?;

        Ok(expenditure)
    }

    pub async fn get_expenditures(&self, category_ids: &[i32]) -> Result<Vec<Expenditure>> {
        let placeholders = category_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ");
        let q = format!(include_str!("sql/get_expenditures.sql"), placeholders);
        let mut query = sqlx::query(&q);
        for id in category_ids {
            query = query.bind(id);
        }
        let expenditures = query
            .map(|row: SqliteRow| {
                Expenditure {
                    id: row.get("id"),
                    description: row.get("description"),
                    vendor: row.get("vendor"),
                    amount: row.get("amount"),
                    category_id: row.get("category_id"),
                }
            })
            .fetch_all(&self.0)
            .await?;

        Ok(expenditures)
    }
}
