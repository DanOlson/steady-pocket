use crate::{
    prelude::*,
    repository::Repository,
    models::{CreateExpenditure, Expenditure, UpdateExpenditure}
};

pub async fn create(repo: &dyn Repository, expenditure: CreateExpenditure) -> Result<Expenditure> {
    repo.create_expenditure(expenditure).await
}

pub async fn update(repo: &dyn Repository, id: i32, expenditure: UpdateExpenditure) -> Result<()> {
    repo.update_expenditure(id, expenditure).await
}
