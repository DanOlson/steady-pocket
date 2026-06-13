import React, { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import ExpenseCategory from './ExpenseCategory'
import BudgetMeter from './components/BudgetMeter'
import apiClient from './api-client'
import NewExpenditureLink from './NewExpenditureLink'
import { formatCurrency } from './format'
import './Budget.css'

export default function () {
  const [budget, setBudget] = useState(null)
  const [categories, setCategories] = useState([])
  const [expenditures, setExpenditures] = useState([])
  const [summary, setSummary] = useState(null)
  const [categorySelections, setCategorySelections] = useState({})
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
  const uncategorizedSpend = summary ? summary.uncategorized_spend_to_date : 0
  const uncategorized = expenditures.filter(expenditure => !expenditure.category_id)

  function setSelectedCategory (expenditureId, categoryId) {
    setCategorySelections({
      ...categorySelections,
      [expenditureId]: Number(categoryId)
    })
  }

  function categorizeExpenditure (expenditure) {
    const expenseCategoryId = categorySelections[expenditure.id]
    if (!expenseCategoryId) {
      return
    }
    apiClient.updateExpenditure({
      id: expenditure.id,
      expenseCategoryId
    }).then(fetchBudget)
  }

  return budget && (
    <div className="budget">
      <h1>{budget.name}</h1>
      <div className="summary">
        <BudgetMeter
          hero
          spent={totalSpend}
          budgeted={totalBudgeted}
          label="Total spent"
        />
      </div>
      {uncategorized.length > 0 && (
        <div className="uncategorized-expenditures">
          <h3>Uncategorized</h3>
          <p>{formatCurrency(uncategorizedSpend / 100)} needs category review.</p>
          <table className="table table-hover">
            <thead>
              <tr>
                <th>Amount</th>
                <th>Description</th>
                <th>Vendor</th>
                <th>Category</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {uncategorized.map(expenditure => (
                <tr key={expenditure.id}>
                  <td>{formatCurrency(expenditure.amount / 100.0)}</td>
                  <td>{expenditure.description}</td>
                  <td>{expenditure.vendor}</td>
                  <td>
                    <select
                      className="form-control"
                      value={categorySelections[expenditure.id] || ''}
                      onChange={e => setSelectedCategory(expenditure.id, e.target.value)}
                    >
                      <option value="">Choose category</option>
                      {categories.map(category => (
                        <option key={category.id} value={category.id}>{category.name}</option>
                      ))}
                    </select>
                  </td>
                  <td>
                    <button
                      className="btn btn-outline-primary btn-sm"
                      type="button"
                      onClick={() => categorizeExpenditure(expenditure)}
                    >
                      Save
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
      <h3>Categories</h3>
      <div className="grid">
        {
          categories.map((category, idx) => {
            return (
              <ExpenseCategory
                key={idx}
                categoryName={category.name}
                amount={category.amount}
                totalSpend={category.total_spend_to_date}
              >
                <div className="actions">
                  <Link
                    className="btn btn-outline-secondary btn-sm"
                    to={`/budgets/${id}/categories/${category.id}`}>
                    Details
                  </Link>
                  <NewExpenditureLink
                    className="btn btn-outline-primary btn-sm"
                    categoryId={category.id}
                    budgetId={id}>
                    Add +
                  </NewExpenditureLink>
                </div>
              </ExpenseCategory>
            )
          })
        }
        <div className="new-category">
          <Link
            to={`/budgets/${id}/categories/new`}
            className="btn btn-outline-secondary"
          >Add Category</Link>
        </div>
      </div>
    </div>
  )
}
