use async_trait::async_trait;
use super::{
    Result,
    Repository,
    Db,
    Budget,
    CreateBudget,
    UpdateBudget,
    ExpenseCategory,
    CreateExpenseCategory,
    UpdateExpenseCategory,
    Expenditure,
    CreateExpenditure,
    UpdateExpenditure,
};

#[derive(Debug, Clone)]
pub struct DatabaseRepository {
    db: Db
}

impl DatabaseRepository {
    pub fn new(db: Db) -> Self {
        DatabaseRepository { db }
    }
}

#[async_trait]
impl Repository for DatabaseRepository {
    async fn budget(&self, id: i32) -> Result<Budget> {
        self.db.get_budget(id).await
    }

    async fn budgets(&self) -> Result<Vec<Budget>> {
        self.db.get_budgets().await
    }

    async fn create_budget(&self, budget: CreateBudget) -> Result<Budget> {
        self.db.create_budget(budget).await
    }

    async fn update_budget(&self, budget_id: i32, budget: UpdateBudget) -> Result<()> {
        self.db.update_budget(budget_id, budget).await
    }

    async fn expense_category(&self, id: i32) -> Result<ExpenseCategory> {
        self.db.get_category(id).await
    }

    async fn expense_categories(&self, budget_id: i32, since: i64) -> Result<Vec<ExpenseCategory>> {
        self.db.get_categories(budget_id, since).await
    }

    async fn create_expense_category(&self, category: CreateExpenseCategory) -> Result<ExpenseCategory> {
        self.db.create_category(category).await
    }

    async fn update_expense_category(&self, id: i32, category: UpdateExpenseCategory) -> Result<()> {
        self.db.update_category(id, category).await
    }

    async fn delete_expense_category(&self, id: i32) -> Result<()> {
        self.db.delete_category(id).await
    }

    async fn expenditure(&self, id: i32) -> Result<Expenditure> {
        self.db.get_expenditure(id).await
    }

    async fn expenditures(&self, category_ids: &[i32], since: i64) -> Result<Vec<Expenditure>> {
        self.db.get_expenditures(category_ids, since).await
    }

    async fn expenditures_since(&self, category_id: i32, since: i64) -> Result<Vec<Expenditure>> {
        self.db.get_expenditures_since(category_id, since).await
    }

    async fn create_expenditure(&self, expenditure: CreateExpenditure) -> Result<Expenditure> {
        self.db.create_expenditure(expenditure).await
    }

    async fn update_expenditure(&self, id: i32, expenditure: UpdateExpenditure) -> Result<()> {
        self.db.update_expenditure(id, expenditure).await
    }

    async fn delete_expenditure(&self, id: i32) -> Result<()> {
        self.db.delete_expenditure(id).await
    }
}
