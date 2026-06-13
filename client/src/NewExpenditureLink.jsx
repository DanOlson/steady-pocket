import React from 'react'
import { Link } from 'react-router-dom'

export default function NewExpenditureLink ({ categoryId, budgetId, className, children }) {
  return (
    <Link
      className={className}
      to={`/budgets/${budgetId}/expenditures/new?categoryId=${categoryId}`}>
      {children || '+'}
    </Link>
  )
}
