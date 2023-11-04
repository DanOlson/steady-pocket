use crate::{
    prelude::*,
    repository::Repository,
    models::{CreateExpenditure, Expenditure}
};

pub async fn create(repo: &dyn Repository, expenditure: CreateExpenditure) -> Result<Expenditure> {
    repo.create_expenditure(expenditure).await
}
