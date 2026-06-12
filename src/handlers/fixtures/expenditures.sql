insert into budgets (id, name, budget_interval) values (1, 'Monthly Budget', 'monthly');
insert into expense_categories (id, name, amount, budget_id, currency) values (1, 'Groceries', 9000, 1, 'USD');
insert into expenditures (
  id,
  description,
  vendor,
  amount,
  budget_id,
  expense_category_id,
  categorization_status,
  effective_date,
  created_at,
  updated_at
) values
  (1, 'Waffles', 'Kroger', 1268, 1, 1, 'confirmed', strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now')),
  (2, 'Sandwich ingredients', 'Publix', 2351, 1, 1, 'confirmed', strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now')),
  (3, 'Fresh fruit', 'Walmart', 1683, 1, 1, 'confirmed', strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now')),
  (4, 'Rotten fruit', 'Walmart', 1683, 1, 1, 'confirmed', strftime('%s', 'now', '-32 days'), strftime('%s', 'now', '-32 days'), strftime('%s', 'now', '-32 days')),
  (5, 'Card charge', 'Uncategorized Store', 4321, 1, null, 'uncategorized', strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now'));
