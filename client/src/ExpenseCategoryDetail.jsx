import React, { useState, useEffect } from 'react'
import { useParams, useNavigate, Link } from 'react-router-dom'
import AppShell from './components/AppShell'
import BudgetMeter from './components/BudgetMeter'
import Button from './components/Button'
import EmptyState from './components/EmptyState'
import ErrorState from './components/ErrorState'
import ListRow from './components/ListRow'
import Sheet from './components/Sheet'
import Skeleton from './components/Skeleton'
import apiClient from './api-client'
import { formatCurrency } from './format'

export default function ExpenseCategoryDetail () {
  const { id, budgetId } = useParams()
  const [category, setCategory] = useState({})
  const [expenditures, setExpenditures] = useState([])
  const [confirmingDelete, setConfirmingDelete] = useState(false)
  const [status, setStatus] = useState('loading')
  const navigate = useNavigate()

  useEffect(fetchData, [id])

  function fetchData () {
    apiClient.getCategory(id)
      .then(json => {
        setCategory(json.category)
        setExpenditures(json.expenditures)
        setStatus('ready')
      })
      .catch(() => setStatus('error'))
  }

  function deleteCategory () {
    apiClient.deleteCategory(category.id)
      .then(() => navigate(`/budgets/${budgetId}`))
      .catch(() => setStatus('error'))
  }

  if (status === 'error') {
    return (
      <AppShell>
        <ErrorState onRetry={fetchData} />
      </AppShell>
    )
  }

  if (status === 'loading') {
    return (
      <AppShell>
        <Skeleton lines={6} />
      </AppShell>
    )
  }

  return (
    <AppShell
      action={
        <Button
          variant="primary"
          className="ui-btn-block"
          to={`/budgets/${budgetId}/expenditures/new?categoryId=${id}`}
        >
          + Record expense
        </Button>
      }
    >
      <Link className="crumb" to={`/budgets/${budgetId}`}>‹ Back to budget</Link>
      <h1 className="screen-title">{category.name}</h1>
      <BudgetMeter
        hero
        spent={category.total_spend_to_date}
        budgeted={category.amount}
        label="Spent"
      />

      <section className="screen-section">
        <div className="screen-section-head">
          <span>Expenses</span>
          <span className="screen-section-count">{expenditures.length}</span>
        </div>
        {expenditures.map(expenditure => (
          <ListRow
            key={expenditure.id}
            to={`/budgets/${budgetId}/expenditures/${expenditure.id}/edit`}
            primary={expenditure.description}
            secondary={[String(expenditure.created_at || '').split('T')[0], expenditure.vendor]
              .filter(Boolean).join(' · ')}
            trailing={formatCurrency(expenditure.amount / 100)}
          />
        ))}
        {expenditures.length === 0 && (
          <EmptyState
            title="No expenses yet"
            message={`Record an expense to start tracking ${category.name || 'this category'}.`}
          />
        )}
      </section>

      <section className="screen-section">
        <div className="screen-section-head">
          <span>Manage</span>
        </div>
        <div className="form-actions">
          <Button
            variant="secondary"
            className="ui-btn-block"
            to={`/budgets/${budgetId}/categories/${id}/edit`}
          >
            Edit category
          </Button>
          <Button
            variant="destructive"
            className="ui-btn-block"
            onClick={() => setConfirmingDelete(true)}
          >
            Delete category
          </Button>
        </div>
      </section>

      <Sheet
        open={confirmingDelete}
        title="Delete category"
        onClose={() => setConfirmingDelete(false)}
      >
        <p>
          Deleting <strong>{category.name}</strong> removes its budget and its
          expenses from this month's totals.
        </p>
        <div className="form-actions">
          <Button variant="destructive" className="ui-btn-block" onClick={deleteCategory}>
            Delete category
          </Button>
          <Button variant="quiet" className="ui-btn-block" onClick={() => setConfirmingDelete(false)}>
            Keep it
          </Button>
        </div>
      </Sheet>
    </AppShell>
  )
}
