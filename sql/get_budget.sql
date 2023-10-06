select b.id, b.name, bi.name as interval_name
from budgets b
join budget_intervals bi
  on b.budget_interval_id = bi.id
where b.id = $budget_id;
