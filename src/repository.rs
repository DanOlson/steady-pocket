use async_trait::async_trait;
use crate::{
    models::{
        Budget,
        CreateBudget,
        UpdateBudget,
        ExpenseCategory,
        CreateExpenseCategory,
        UpdateExpenseCategory,
        Expenditure,
        CreateExpenditure,
        UpdateExpenditure,
    },
    db::Db,
    prelude::*
};

mod db_repo;
mod memory_repo;

pub use db_repo::DatabaseRepository;
pub use memory_repo::MemoryRepository;

#[async_trait]
pub trait Repository {
    async fn budget(&self, id: i32) -> Result<Budget>;
    async fn budgets(&self) -> Result<Vec<Budget>>;
    async fn create_budget(&self, budget: CreateBudget) -> Result<Budget>;
    async fn update_budget(&self, budget_id: i32, budget: UpdateBudget) -> Result<()>;
    async fn expense_category(&self, id: i32) -> Result<ExpenseCategory>;
    async fn expense_categories(&self, budget_id: i32, since: i64) -> Result<Vec<ExpenseCategory>>;
    async fn create_expense_category(&self, category: CreateExpenseCategory) -> Result<ExpenseCategory>;
    async fn update_expense_category(&self, id: i32, category: UpdateExpenseCategory) -> Result<()>;
    async fn delete_expense_category(&self, id: i32) -> Result<()>;
    async fn expenditure(&self, expenditure_id: i32) -> Result<Expenditure>;
    async fn expenditures(&self, category_ids: &[i32], since: i64) -> Result<Vec<Expenditure>>;
    async fn expenditures_since(&self, category_id: i32, since: i64) -> Result<Vec<Expenditure>>;
    async fn create_expenditure(&self, expenditure: CreateExpenditure) -> Result<Expenditure>;
    async fn update_expenditure(&self, id: i32, expenditure: UpdateExpenditure) -> Result<()>;
    async fn delete_expenditure(&self, id: i32) -> Result<()>;
}
