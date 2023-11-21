select id
      ,description
      ,vendor
      ,amount
      ,effective_date
      ,expense_category_id as category_id
      ,created_at
      ,updated_at
from expenditures
where category_id = ?
  and effective_date >= ?;
