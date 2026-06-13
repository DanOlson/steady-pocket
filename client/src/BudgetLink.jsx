import React from 'react'
import { Link } from 'react-router-dom'

export default function BudgetLink ({ budgetId, children, className }) {
  return <Link className={className} to={`/budgets/${budgetId}`}>{children}</Link>
}
