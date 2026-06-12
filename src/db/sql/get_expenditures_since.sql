select id
      ,description
      ,vendor
      ,amount
      ,effective_date
      ,budget_id
      ,expense_category_id as category_id
      ,categorization_status
      ,created_at
      ,updated_at
from expenditures
where expense_category_id = ?
  and effective_date >= ?;
