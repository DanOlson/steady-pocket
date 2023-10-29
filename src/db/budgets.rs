use crate::{
    prelude::*,
    db::Db,
    models::{
        Budget,
        CreateBudget,
        UpdateBudget
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
}
