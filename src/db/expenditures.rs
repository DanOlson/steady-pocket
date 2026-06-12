use crate::{
    db::Db,
    models::{CreateExpenditure, Expenditure, NullableUpdate, UpdateExpenditure},
    prelude::*,
};
use sqlx::{sqlite::SqliteRow, QueryBuilder, Row};

fn map_expenditure(row: SqliteRow) -> Expenditure {
    Expenditure {
        id: row.get("id"),
        description: row.get("description"),
        vendor: row.get("vendor"),
        amount: row.get("amount"),
        budget_id: row.get("budget_id"),
        category_id: row.get("category_id"),
        effective_date: row.get("effective_date"),
        categorization_status: row.get("categorization_status"),
        created_at: row.get("created_at"),
    }
}

impl Db {
    pub async fn get_expenditure(&self, id: i32) -> Result<Expenditure> {
        let q = include_str!("sql/get_expenditure.sql");
        let expenditure = sqlx::query(q)
            .bind(id)
            .map(map_expenditure)
            .fetch_one(&self.0)
            .await?;

        Ok(expenditure)
    }

    pub async fn get_expenditures(
        &self,
        category_ids: &[i32],
        since: i64,
    ) -> Result<Vec<Expenditure>> {
        if category_ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = category_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ");
        let q = format!(include_str!("sql/get_expenditures.sql"), placeholders);
        let mut query = sqlx::query(&q).bind(since);
        for id in category_ids {
            query = query.bind(id);
        }
        let expenditures = query.map(map_expenditure).fetch_all(&self.0).await?;

        Ok(expenditures)
    }

    pub async fn get_budget_expenditures(
        &self,
        budget_id: i32,
        since: i64,
        categorized: Option<bool>,
    ) -> Result<Vec<Expenditure>> {
        let mut builder = QueryBuilder::new(
            "select id, description, vendor, amount, effective_date, budget_id, \
             expense_category_id as category_id, categorization_status, created_at, updated_at \
             from expenditures where budget_id = ",
        );
        builder.push_bind(budget_id);
        builder.push(" and effective_date >= ");
        builder.push_bind(since);
        if let Some(categorized) = categorized {
            if categorized {
                builder.push(" and expense_category_id is not null");
            } else {
                builder.push(" and expense_category_id is null");
            }
        }
        builder.push(" order by effective_date desc, id desc");

        let expenditures = builder
            .build()
            .map(map_expenditure)
            .fetch_all(&self.0)
            .await?;

        Ok(expenditures)
    }

    pub async fn get_expenditures_since(
        &self,
        category_id: i32,
        since: i64,
    ) -> Result<Vec<Expenditure>> {
        let q = include_str!("sql/get_expenditures_since.sql");
        let expenditures = sqlx::query(q)
            .bind(category_id)
            .bind(since)
            .map(map_expenditure)
            .fetch_all(&self.0)
            .await?;
        Ok(expenditures)
    }

    pub async fn create_expenditure(&self, expenditure: CreateExpenditure) -> Result<Expenditure> {
        let q = include_str!("sql/create_expenditure.sql");
        let categorization_status = if expenditure.expense_category_id.is_some() {
            "confirmed"
        } else {
            "uncategorized"
        };
        let expenditure = sqlx::query(q)
            .bind(expenditure.description)
            .bind(expenditure.vendor)
            .bind(expenditure.amount)
            .bind(expenditure.budget_id)
            .bind(expenditure.expense_category_id)
            .bind(categorization_status)
            .bind(expenditure.effective_date)
            .map(map_expenditure)
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
        match expenditure.expense_category_id {
            NullableUpdate::Value(expense_category_id) => {
                separator.push("expense_category_id = ");
                separator.push_bind_unseparated(expense_category_id);
                separator.push("categorization_status = ");
                separator.push_bind_unseparated("confirmed");
            }
            NullableUpdate::Null => {
                separator.push("expense_category_id = null");
                separator.push("categorization_status = ");
                separator.push_bind_unseparated("uncategorized");
            }
            NullableUpdate::Unset => {}
        }
        if let Some(effective_date) = expenditure.effective_date {
            separator.push("effective_date = ");
            separator.push_bind_unseparated(effective_date);
        }
        separator.push("updated_at = strftime('%s', 'now')");
        builder.push(" where id = ");
        builder.push_bind(id);

        builder.build().execute(&self.0).await?;

        Ok(())
    }

    pub async fn delete_expenditure(&self, id: i32) -> Result<()> {
        let q = include_str!("sql/delete_expenditure.sql");
        sqlx::query(q).bind(id).execute(&self.0).await?;
        Ok(())
    }
}
