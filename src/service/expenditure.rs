use crate::{
    models::{
        CreateExpenditure, Expenditure, ExpendituresQuery, NullableUpdate, UpdateExpenditure,
    },
    prelude::*,
    repository::Repository,
    util::time_util,
};

pub async fn for_query(
    repo: &dyn Repository,
    query: ExpendituresQuery,
) -> Result<Vec<Expenditure>> {
    let since = time_util::start_of_current_month();
    if let Some(expense_category_id) = query.expense_category_id {
        repo.expenditures(&[expense_category_id], since).await
    } else if let Some(budget_id) = query.budget_id {
        repo.budget_expenditures(budget_id, since, query.categorized)
            .await
    } else {
        Err(Error::BadRequest(
            "expense_category_id or budget_id query parameter is required".to_string(),
        ))
    }
}

pub async fn find(repo: &dyn Repository, id: i32) -> Result<Expenditure> {
    repo.expenditure(id).await
}

pub async fn create(repo: &dyn Repository, expenditure: CreateExpenditure) -> Result<Expenditure> {
    if let Some(category_id) = expenditure.expense_category_id {
        validate_category_belongs_to_budget(repo, category_id, expenditure.budget_id).await?;
    } else {
        repo.budget(expenditure.budget_id).await?;
    }

    repo.create_expenditure(expenditure).await
}

pub async fn update(repo: &dyn Repository, id: i32, expenditure: UpdateExpenditure) -> Result<()> {
    if let NullableUpdate::Value(category_id) = &expenditure.expense_category_id {
        let existing = repo.expenditure(id).await?;
        validate_category_belongs_to_budget(repo, *category_id, existing.budget_id).await?;
    }

    repo.update_expenditure(id, expenditure).await
}

pub async fn delete(repo: &dyn Repository, id: i32) -> Result<()> {
    repo.delete_expenditure(id).await
}

async fn validate_category_belongs_to_budget(
    repo: &dyn Repository,
    category_id: i32,
    budget_id: i32,
) -> Result<()> {
    let category = repo.expense_category(category_id).await?;
    if category.budget_id == budget_id {
        Ok(())
    } else {
        Err(Error::BadRequest(format!(
            "expense_category_id {category_id} does not belong to budget_id {budget_id}"
        )))
    }
}
