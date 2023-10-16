use async_trait::async_trait;
use crate::{
    models::{
        Budget,
        CreateBudget,
        ExpenseCategory,
        Expenditure
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
    async fn expense_category(&self, id: i32) -> Result<ExpenseCategory>;
    async fn expense_categories(&self, budget_id: i32) -> Result<Vec<ExpenseCategory>>;
    async fn expenditure(&self, expenditure_id: i32) -> Result<Expenditure>;
    async fn expenditures(&self, category_ids: &[i32]) -> Result<Vec<Expenditure>>;
}
