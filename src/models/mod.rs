use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

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

#[derive(Serialize)]
pub struct BudgetDTO {
    pub id: i32,
    pub name: String,
    pub category_ids: Vec<i32>,
}

#[derive(Serialize)]
pub struct BudgetResponse {
    pub budget: BudgetDTO,
    pub categories: Vec<ExpenseCategory>,
}
