use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Budget {
    pub id: i32,
    pub name: String,
    pub interval_name: String,
}

#[derive(Serialize)]
pub struct ExpenseCategory {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub budget_id: i32,
    pub total_spend_to_date: i64,
    pub expenditure_ids: Vec<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct Expenditure {
    pub id: i32,
    pub description: String,
    pub amount: i32,
    pub vendor: String,
    pub category_id: i32
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
    pub expenditures: Vec<Expenditure>,
}

