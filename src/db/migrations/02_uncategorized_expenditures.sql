CREATE TABLE expenditures_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT,
    vendor TEXT,
    amount INTEGER NOT NULL,
    currency TEXT,
    effective_date INTEGER NOT NULL,
    budget_id INTEGER NOT NULL,
    expense_category_id INTEGER,
    categorization_status TEXT NOT NULL DEFAULT 'uncategorized',
    created_at INTEGER,
    updated_at INTEGER
);

INSERT INTO expenditures_new (
    id,
    description,
    vendor,
    amount,
    currency,
    effective_date,
    budget_id,
    expense_category_id,
    categorization_status,
    created_at,
    updated_at
)
SELECT expenditures.id,
       expenditures.description,
       expenditures.vendor,
       expenditures.amount,
       expenditures.currency,
       coalesce(expenditures.effective_date, expenditures.created_at, strftime('%s', 'now')),
       expense_categories.budget_id,
       expenditures.expense_category_id,
       'confirmed',
       expenditures.created_at,
       expenditures.updated_at
FROM expenditures
LEFT JOIN expense_categories
  ON expenditures.expense_category_id = expense_categories.id;

DROP TABLE expenditures;
ALTER TABLE expenditures_new RENAME TO expenditures;
