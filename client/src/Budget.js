import React, { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import ExpenseCategory from './ExpenseCategory'
import Gauge from './Gauge'
import apiClient from './api-client'
import NewExpenditureLink from './NewExpenditureLink'
import { format } from 'd3-format'
import './Budget.css'

export default function () {
  const [budget, setBudget] = useState(null)
  const [categories, setCategories] = useState([])
  const { id } = useParams()

  useEffect(() => {
    apiClient.getBudget(id)
      .then(json => {
        setBudget(json.budget)
        setCategories(json.categories)
      })
  }, [id])

  const totals = categories.reduce((acc, category) => {
    acc.amount = (acc.amount || 0) + category.amount / 100.0
    acc.spendToDate = (acc.spendToDate || 0) + category.total_spend_to_date / 100.0
    return acc
  }, {})

  return budget && (
    <div className="budget">
      <h1>{budget.name}</h1>
      <div className="summary">
        <Gauge
          max={totals.amount}
          value={totals.spendToDate}
          label="Total Spend"
          units={`out of ${format("$,")(totals.amount)}`}
        />
      </div>
      <h3>Categories</h3>
      <div className="grid">
        {
          categories.map((category, idx) => {
            return (
              <ExpenseCategory
                key={idx}
                categoryName={category.name}
                amount={category.amount / 100.0}
                totalSpend={category.total_spend_to_date / 100.0}
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
