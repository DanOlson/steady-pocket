use crate::{
    errors::MyError,
    models::{
        Budget,
        ExpenseCategoryDTO,
        Expenditure
    }
};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_budgets(client: &Client) -> Result<Vec<Budget>, MyError> {
    let q = include_str!("../../sql/get_budgets.sql");
    let query = client.prepare(q).await.unwrap();

    let res = client.query(&query, &[])
        .await?
        .iter()
        .map(|row| Budget::from_row_ref(row).unwrap())
        .collect::<Vec<Budget>>();

    Ok(res)
}

pub async fn get_budget(client: &Client, budget_id: i32) -> Result<Budget, MyError> {
    let q = include_str!("../../sql/get_budget.sql");
    let q = q.replace("$budget_id", budget_id.to_string().as_str());
    let query = client.prepare(&q).await.unwrap();

    client.query(&query, &[])
        .await?
        .iter()
        .map(|row| Budget::from_row_ref(row).unwrap())
        .collect::<Vec<Budget>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn get_categories(client: &Client, budget_id: i32) -> Result<Vec<ExpenseCategoryDTO>, MyError> {
    let q = include_str!("../../sql/get_budget_expense_categories.sql");
    // let q = q.replace("$table_fields", &ExpenseCategory::sql_table_fields());
    let q = q.replace("$budget_id", budget_id.to_string().as_str());
    let query = client.prepare(&q).await.unwrap();

    let res = client.query(&query, &[])
        .await?
        .iter()
        .map(|row| ExpenseCategoryDTO::from(row))
        .collect::<Vec<ExpenseCategoryDTO>>();

    Ok(res)
}

pub async fn get_expenditures(client: &Client, category_ids: &Vec<i32>) -> Result<Vec<Expenditure>, MyError> {
    let q = include_str!("../../sql/get_expenditures.sql");
    let sub = category_ids
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let q = q.replace("$category_ids", &sub);
    let query = client.prepare(&q).await.unwrap();
    let res = client.query(&query, &[])
        .await?
        .iter()
        .map(|row| Expenditure::from_row_ref(row).unwrap())
        .collect::<Vec<Expenditure>>();
    Ok(res)
}
