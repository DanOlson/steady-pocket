select b.id, b.name, b.budget_interval as interval_name
from budgets b
where b.id = ?;
