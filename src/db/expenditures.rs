use crate::{
    prelude::*,
    db::Db,
    models::Expenditure
};
use sqlx::{sqlite::SqliteRow, Row};

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
}
