insert into expenditures (
  description,
  vendor,
  amount,
  expense_category_id,
  effective_date,
  created_at,
  updated_at
) values (
  ?,
  ?,
  ?,
  ?,
  strftime('%s', 'now'),
  strftime('%s', 'now'),
  strftime('%s', 'now')
) returning *;
