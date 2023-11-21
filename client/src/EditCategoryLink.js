import React from 'react'
import { Link } from 'react-router-dom'

export default function EditCategoryLink ({ budgetId, categoryId, children, className }) {
  return (
    <Link
      to={`/budgets/${budgetId}/categories/${categoryId}/edit`}
      className={className}
    >{children}</Link>
  )
}
