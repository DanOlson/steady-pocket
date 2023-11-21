use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Budget {
    pub id: i32,
    pub name: String,
    pub interval_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBudgetsDTO {
    pub budgets: Vec<Budget>
}

#[derive(Deserialize, Serialize)]
pub struct CreateBudgetDTO {
    pub budget: CreateBudget
}

#[derive(Deserialize, Serialize)]
pub struct CreateBudget {
    pub name: String,
    pub interval_name: String
}

#[derive(Deserialize, Serialize)]
pub struct UpdateBudgetDTO {
    pub budget: UpdateBudget
}

#[derive(Deserialize, Serialize)]
pub struct UpdateBudget {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct ExpenseCategory {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub budget_id: i32,
    pub total_spend_to_date: i64,
    pub expenditure_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct GetExpenseCategoryDTO {
    pub category: ExpenseCategory,
    pub expenditures: Vec<Expenditure>
}

#[derive(Serialize, Deserialize)]
pub struct CreateExpenseCategoryDTO {
    pub category: CreateExpenseCategory
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExpenseCategoryDTO {
    pub category: UpdateExpenseCategory
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExpenseCategory {
    pub name: Option<String>,
    pub amount: Option<i32>
}

#[derive(Serialize, Deserialize)]
pub struct CreateExpenseCategory {
    pub name: String,
    pub amount: i32,
    pub budget_id: i32
}

#[derive(Deserialize, Serialize)]
pub struct Expenditure {
    pub id: i32,
    pub description: String,
    pub amount: i32,
    pub vendor: String,
    pub category_id: i32,
    pub created_at: i64
}

#[derive(Serialize, Deserialize)]
pub struct GetExpenditureDTO {
    pub expenditure: Expenditure
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

#[derive(Deserialize)]
pub struct CreateExpenditureDTO {
    pub expenditure: CreateExpenditure
}

#[derive(Deserialize)]
pub struct CreateExpenditure {
    pub amount: i32,
    pub vendor: String,
    pub description: String,
    pub expense_category_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateExpenditureDTO {
    pub expenditure: UpdateExpenditure
}

#[derive(Deserialize)]
pub struct UpdateExpenditure {
    pub amount: Option<i32>,
    pub description: Option<String>,
    pub vendor: Option<String>,
}

#[derive(Deserialize)]
pub struct ExpendituresQuery {
    pub expense_category_id: i32
}
