use crate::{
    prelude::*,
    db::Db,
    models::{Expenditure, CreateExpenditure, UpdateExpenditure}
};
use sqlx::{sqlite::SqliteRow, Row, QueryBuilder};

impl Db {
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

    pub async fn create_expenditure(&self, expenditure: CreateExpenditure) -> Result<Expenditure> {
        let q = include_str!("sql/create_expenditure.sql");
        let expenditure = sqlx::query(q)
            .bind(expenditure.description)
            .bind(expenditure.vendor)
            .bind(expenditure.amount)
            .bind(expenditure.expense_category_id)
            .map(|row: SqliteRow| {
                Expenditure {
                    id: row.get("id"),
                    description: row.get("description"),
                    amount: row.get("amount"),
                    vendor: row.get("vendor"),
                    category_id: row.get("expense_category_id")
                }
            })
            .fetch_one(&self.0)
            .await?;

        Ok(expenditure)
    }

    pub async fn update_expenditure(&self, id: i32, expenditure: UpdateExpenditure) -> Result<()> {
        let mut builder = QueryBuilder::new("update expenditures set ");
        let mut separator = builder.separated(", ");

        if let Some(description) = expenditure.description {
            separator.push("description = ");
            separator.push_bind_unseparated(description);
        }
        if let Some(vendor) = expenditure.vendor {
            separator.push("vendor = ");
            separator.push_bind_unseparated(vendor);
        }
        if let Some(amount) = expenditure.amount {
            separator.push("amount = ");
            separator.push_bind_unseparated(amount);
        }
        builder.push("where id = ");
        builder.push_bind(id);

        builder.build()
            .execute(&self.0)
            .await?;

        Ok(())
    }
}
