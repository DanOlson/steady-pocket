use crate::{
    prelude::*,
    repository::Repository,
    util::time_util,
    models::{
        CreateExpenditure,
        Expenditure,
        UpdateExpenditure,
        ExpendituresQuery
    }
};

pub async fn for_query(repo: &dyn Repository, query: ExpendituresQuery) -> Result<Vec<Expenditure>> {
    let since = time_util::start_of_current_month();
    repo.expenditures(&[query.expense_category_id], since).await
}

pub async fn find(repo: &dyn Repository, id: i32) -> Result<Expenditure> {
    repo.expenditure(id).await
}

pub async fn create(repo: &dyn Repository, expenditure: CreateExpenditure) -> Result<Expenditure> {
    repo.create_expenditure(expenditure).await
}

pub async fn update(repo: &dyn Repository, id: i32, expenditure: UpdateExpenditure) -> Result<()> {
    repo.update_expenditure(id, expenditure).await
}

pub async fn delete(repo: &dyn Repository, id: i32) -> Result<()> {
    repo.delete_expenditure(id).await
}
