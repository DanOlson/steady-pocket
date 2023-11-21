insert into budgets (id, name, budget_interval) values (1, 'Monthly Budget', 'monthly');
insert into expense_categories (id, name, amount, budget_id, currency) values (1, 'Groceries', 9000, 1, 'USD');
insert into expenditures (
  id,
  description,
  vendor,
  amount,
  expense_category_id,
  effective_date,
  created_at,
  updated_at
) values
  (1, 'Waffles', 'Kroger', 1268, 1, strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now')),
  (2, 'Sandwich ingredients', 'Publix', 2351, 1, strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now')),
  (3, 'Fresh fruit', 'Walmart', 1683, 1, strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now'));
