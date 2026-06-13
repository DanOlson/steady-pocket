import React, { useEffect, useState } from 'react'
import AppShell from './components/AppShell'
import EmptyState from './components/EmptyState'
import ListRow from './components/ListRow'
import apiClient from './api-client'

export default function BudgetList () {
  const [budgets, setBudgets] = useState([])

  useEffect(() => {
    apiClient.getBudgets()
      .then(json => {
        setBudgets(json.budgets)
      })
  }, [setBudgets])

  return (
    <AppShell>
      <h1 className="screen-title">Budgets</h1>
      {budgets.map(budget => (
        <ListRow
          key={budget.id}
          to={`/budgets/${budget.id}`}
          primary={budget.name}
        />
      ))}
      {budgets.length === 0 && (
        <EmptyState
          title="No budgets yet"
          message="Budgets are created through the API for now."
        />
      )}
    </AppShell>
  )
}
