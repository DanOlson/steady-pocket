select id
      ,name
      ,budget_interval as interval_name
from budgets
where id = ?;
