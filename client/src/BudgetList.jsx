import React, { useEffect, useState } from 'react'
import AppShell from './components/AppShell'
import EmptyState from './components/EmptyState'
import ErrorState from './components/ErrorState'
import ListRow from './components/ListRow'
import Skeleton from './components/Skeleton'
import apiClient from './api-client'

export default function BudgetList () {
  const [budgets, setBudgets] = useState([])
  const [status, setStatus] = useState('loading')

  useEffect(fetchBudgets, [])

  function fetchBudgets () {
    apiClient.getBudgets()
      .then(json => {
        setBudgets(json.budgets)
        setStatus('ready')
      })
      .catch(() => setStatus('error'))
  }

  return (
    <AppShell>
      <h1 className="screen-title">Budgets</h1>
      {status === 'error' && <ErrorState onRetry={fetchBudgets} />}
      {status === 'loading' && <Skeleton />}
      {status === 'ready' && budgets.map(budget => (
        <ListRow
          key={budget.id}
          to={`/budgets/${budget.id}`}
          primary={budget.name}
        />
      ))}
      {status === 'ready' && budgets.length === 0 && (
        <EmptyState
          title="No budgets yet"
          message="Budgets are created through the API for now."
        />
      )}
    </AppShell>
  )
}
