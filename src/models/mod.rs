use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq)]
pub enum NullableUpdate<T> {
    Unset,
    Null,
    Value(T),
}

impl<T> Default for NullableUpdate<T> {
    fn default() -> Self {
        NullableUpdate::Unset
    }
}

fn deserialize_nullable_update<'de, D, T>(
    deserializer: D,
) -> std::result::Result<NullableUpdate<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(|value| match value {
        Some(value) => NullableUpdate::Value(value),
        None => NullableUpdate::Null,
    })
}

#[derive(Deserialize, Serialize)]
pub struct Budget {
    pub id: i32,
    pub name: String,
    pub interval_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBudgetsDTO {
    pub budgets: Vec<Budget>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateBudgetDTO {
    pub budget: CreateBudget,
}

#[derive(Deserialize, Serialize)]
pub struct CreateBudget {
    pub name: String,
    pub interval_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateBudgetDTO {
    pub budget: UpdateBudget,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateBudget {
    pub name: String,
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
    pub expenditures: Vec<Expenditure>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateExpenseCategoryDTO {
    pub category: CreateExpenseCategory,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExpenseCategoryDTO {
    pub category: UpdateExpenseCategory,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExpenseCategory {
    pub name: Option<String>,
    pub amount: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateExpenseCategory {
    pub name: String,
    pub amount: i32,
    pub budget_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Expenditure {
    pub id: i32,
    pub description: String,
    pub amount: i32,
    pub vendor: String,
    pub budget_id: i32,
    pub category_id: Option<i32>,
    pub effective_date: i64,
    pub categorization_status: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetExpenditureDTO {
    pub expenditure: Expenditure,
}

#[derive(Serialize, Deserialize)]
pub struct BudgetDTO {
    pub id: i32,
    pub name: String,
    pub category_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct BudgetResponse {
    pub budget: BudgetDTO,
    pub summary: BudgetSummary,
    pub categories: Vec<ExpenseCategory>,
    pub expenditures: Vec<Expenditure>,
}

#[derive(Serialize, Deserialize)]
pub struct BudgetSummary {
    pub budgeted_amount: i64,
    pub total_spend_to_date: i64,
    pub categorized_spend_to_date: i64,
    pub uncategorized_spend_to_date: i64,
}

#[derive(Deserialize)]
pub struct CreateExpenditureDTO {
    pub expenditure: CreateExpenditure,
}

#[derive(Deserialize)]
pub struct CreateExpenditure {
    pub amount: i32,
    pub vendor: String,
    pub description: String,
    pub budget_id: i32,
    pub expense_category_id: Option<i32>,
    pub effective_date: Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdateExpenditureDTO {
    pub expenditure: UpdateExpenditure,
}

#[derive(Deserialize)]
pub struct UpdateExpenditure {
    pub amount: Option<i32>,
    pub description: Option<String>,
    pub vendor: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nullable_update")]
    pub expense_category_id: NullableUpdate<i32>,
    pub effective_date: Option<i64>,
}

#[derive(Deserialize)]
pub struct ExpendituresQuery {
    pub expense_category_id: Option<i32>,
    pub budget_id: Option<i32>,
    pub categorized: Option<bool>,
}
