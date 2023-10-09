use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "budgets")]
pub struct Budget {
    pub id: i32,
    pub name: String,
    pub interval_name: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "expense_categories")]
pub struct ExpenseCategory {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub budget_id: i32,
}

#[derive(Serialize, PostgresMapper)]
#[pg_mapper(table = "expense_categories")]
pub struct ExpenseCategoryDTO {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub budget_id: i32,
    pub total_spend_to_date: i64,
    pub expenditure_ids: Vec<i32>,
}

impl From<&Row> for ExpenseCategoryDTO {
    fn from(row: &Row) -> Self {
        ExpenseCategoryDTO {
            id: row.get(0),
            name: row.get(1),
            amount: row.get(2),
            budget_id: row.get(3),
            total_spend_to_date: row.get(4),
            expenditure_ids: row.get(5),
        }
    }
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "expense_categories")]
pub struct Expenditure {
    pub id: i32,
    pub description: String,
    pub amount: i32,
    pub vendor: String,
    pub category_id: i32
}

impl From<&Row> for Expenditure {
    fn from(row: &Row) -> Self {
        Expenditure {
            id: row.get("id"),
            description: row.get("description"),
            vendor: row.get("vendor"),
            amount: row.get("amount"),
            category_id: row.get("category_id")
        }
    }
}

#[derive(Serialize)]
pub struct BudgetDTO {
    pub id: i32,
    pub name: String,
    pub category_ids: Vec<i32>,
}

#[derive(Serialize)]
pub struct BudgetResponse {
    pub budget: BudgetDTO,
    pub categories: Vec<ExpenseCategoryDTO>,
    pub expenditures: Vec<Expenditure>,
}

