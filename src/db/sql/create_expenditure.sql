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
  current_timestamp,
  current_timestamp,
  current_timestamp
) returning *;
