CREATE TABLE budgets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    budget_interval TEXT
);

CREATE TABLE budgets_expense_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    budget_id INTEGER,
    expense_category_id INTEGER,
    amount INTEGER NOT NULL,
    time_period TEXT NOT NULL,
    activated_at INTEGER NOT NULL,
    deactivated_at INTEGER
);

CREATE TABLE expenditures (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT,
    vendor TEXT,
    amount INTEGER NOT NULL,
    currency TEXT,
    effective_date INTEGER,
    expense_category_id INTEGER NOT NULL,
    created_at INTEGER,
    updated_at INTEGER
);

CREATE TABLE expense_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    budget_id INTEGER NOT NULL,
    amount INTEGER,
    currency TEXT,
    created_at INTEGER,
    updated_at INTEGER
);
