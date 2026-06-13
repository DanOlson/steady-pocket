import React from 'react'
import { Routes, Route, Navigate } from 'react-router-dom'
import BudgetList from './BudgetList'
import Budget from './Budget'
import NewExpenditure from './NewExpenditure'
import EditExpenditure from './EditExpenditure'
import NewCategory from './NewCategory'
import ExpenseCategoryDetail from './ExpenseCategoryDetail'
import EditExpenseCategory from './EditExpenseCategory'
import Design from './Design'

export default function AppRoutes () {
  return (
    <Routes>
      <Route path="/" element={<Navigate to="/budgets" replace />} />
      <Route path="/budgets" element={<BudgetList />} />
      <Route path="/budgets/:id" element={<Budget />} />
      <Route path="/budgets/:budgetId/expenditures/new" element={<NewExpenditure />} />
      <Route path="/budgets/:budgetId/expenditures/:id/edit" element={<EditExpenditure />} />
      <Route path="/budgets/:budgetId/categories/new" element={<NewCategory />} />
      <Route path="/budgets/:budgetId/categories/:id" element={<ExpenseCategoryDetail />} />
      <Route path="/budgets/:budgetId/categories/:id/edit" element={<EditExpenseCategory />} />
      <Route path="/design" element={<Design />} />
    </Routes>
  )
}
