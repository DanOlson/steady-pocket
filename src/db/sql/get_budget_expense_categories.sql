select expense_categories.id
      ,expense_categories.name
      ,expense_categories.amount
      ,expense_categories.budget_id
      ,cast(total(expenditures.amount) as integer) as total_spend_to_date
      ,group_concat(coalesce(expenditures.id, ''), ' ') as expenditure_ids
from expense_categories
left join expenditures
  on expenditures.expense_category_id = expense_categories.id
where budget_id = ?
group by expense_categories.id;
