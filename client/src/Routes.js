import React from 'react'
import { Switch, Route } from 'react-router-dom'
import BudgetList from './BudgetList'
import Budget from './Budget'
import NewExpenditure from './NewExpenditure'
import EditExpenditure from './EditExpenditure'
import NewCategory from './NewCategory'
import ExpenseCategoryDetail from './ExpenseCategoryDetail'
import EditExpenseCategory from './EditExpenseCategory'

export default function Routes () {
  return (
    <Switch>
      <Route exact path="/budgets">
        <BudgetList />
      </Route>
      <Route exact path="/budgets/:id">
        <Budget />
      </Route>
      <Route exact path="/budgets/:budgetId/expenditures/new">
        <NewExpenditure />
      </Route>
      <Route exact path="/budgets/:budgetId/expenditures/:id/edit">
        <EditExpenditure />
      </Route>
      <Route exact path="/budgets/:budgetId/categories/new">
        <NewCategory />
      </Route>
      <Route exact path="/budgets/:budgetId/categories/:id">
        <ExpenseCategoryDetail />
      </Route>
      <Route exact path="/budgets/:budgetId/categories/:id/edit">
        <EditExpenseCategory />
      </Route>
    </Switch>
  )
}
