select expense_categories.id
      ,expense_categories.name
      ,expense_categories.amount
      ,expense_categories.budget_id
      ,coalesce(sum(expenditures.amount), 0::bigint) as total_spend_to_date
      ,array_remove(array_agg(expenditures.id), NULL) as expenditure_ids
from expense_categories
left join expenditures
  on expenditures.expense_category_id = expense_categories.id
where budget_id = $budget_id
group by expense_categories.id;
