import React from 'react'
import { Link } from 'react-router-dom'

export default function EditExpenditureLink ({ budgetId, expenditure }) {
  return (
    <Link
      to={`/budgets/${budgetId}/expenditures/${expenditure.id}/edit`}
      className="btn btn-outline-primary btn-sm">Edit</Link>
  )
}
