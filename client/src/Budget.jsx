import React, { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import AppShell from './components/AppShell'
import BudgetMeter from './components/BudgetMeter'
import Button from './components/Button'
import Card from './components/Card'
import EmptyState from './components/EmptyState'
import { SelectField } from './components/Field'
import apiClient from './api-client'
import { formatCurrency } from './format'
import './Budget.css'

export default function Budget () {
  const [budget, setBudget] = useState(null)
  const [categories, setCategories] = useState([])
  const [expenditures, setExpenditures] = useState([])
  const [summary, setSummary] = useState(null)
  const { id } = useParams()

  useEffect(fetchBudget, [id])

  function fetchBudget () {
    apiClient.getBudget(id)
      .then(json => {
        setBudget(json.budget)
        setCategories(json.categories)
        setExpenditures(json.expenditures || [])
        setSummary(json.summary)
      })
  }

  // All amounts in cents, as the API returns them
  const totals = categories.reduce((acc, category) => {
    acc.amount = (acc.amount || 0) + category.amount
    acc.spendToDate = (acc.spendToDate || 0) + category.total_spend_to_date
    return acc
  }, {})
  const totalBudgeted = summary ? summary.budgeted_amount : totals.amount
  const totalSpend = summary ? summary.total_spend_to_date : totals.spendToDate
  const uncategorized = expenditures.filter(expenditure => !expenditure.category_id)

  const today = new Date()
  const monthLabel = today.toLocaleString('en-US', { month: 'long', year: 'numeric' })
  const daysInMonth = new Date(today.getFullYear(), today.getMonth() + 1, 0).getDate()

  function fileExpenditure (expenditure, categoryId) {
    if (!categoryId) {
      return
    }
    apiClient.updateExpenditure({
      id: expenditure.id,
      expenseCategoryId: Number(categoryId)
    }).then(fetchBudget)
  }

  return budget && (
    <AppShell
      action={
        <Button variant="primary" className="ui-btn-block" to={`/budgets/${id}/expenditures/new`}>
          + Record expense
        </Button>
      }
    >
      <header className="budget-header">
        <h1>{budget.name}</h1>
        <p className="budget-period">{monthLabel} · Day {today.getDate()} of {daysInMonth}</p>
      </header>

      <BudgetMeter hero spent={totalSpend} budgeted={totalBudgeted} label="Total spent" />

      {uncategorized.length > 0 && (
        <section className="budget-section">
          <div className="budget-section-head">
            <span>Needs filing</span>
            <span className="budget-section-count">{uncategorized.length}</span>
          </div>
          {uncategorized.map(expenditure => (
            <Card className="filing-card" key={expenditure.id}>
              <div className="filing-line">
                <span className="filing-amount">{formatCurrency(expenditure.amount / 100)}</span>
                <span className="filing-desc">{expenditure.vendor} · {expenditure.description}</span>
              </div>
              <SelectField
                label="Category"
                name={`category-${expenditure.id}`}
                defaultValue=""
                onChange={e => fileExpenditure(expenditure, e.target.value)}
              >
                <option value="">Choose a category</option>
                {categories.map(category => (
                  <option key={category.id} value={category.id}>{category.name}</option>
                ))}
              </SelectField>
            </Card>
          ))}
        </section>
      )}

      <section className="budget-section">
        <div className="budget-section-head">
          <span>Categories</span>
          <span>{today.toLocaleString('en-US', { month: 'long' })}</span>
        </div>
        {categories.map(category => (
          <Link
            key={category.id}
            className="budget-category-row"
            to={`/budgets/${id}/categories/${category.id}`}
          >
            <BudgetMeter
              label={category.name}
              spent={category.total_spend_to_date}
              budgeted={category.amount}
            />
          </Link>
        ))}
        {categories.length === 0 && (
          <EmptyState
            title="No categories yet"
            message="Add a category to start budgeting this month."
          />
        )}
        <Button variant="quiet" className="ui-btn-block" to={`/budgets/${id}/categories/new`}>
          + Add a category
        </Button>
      </section>
    </AppShell>
  )
}
