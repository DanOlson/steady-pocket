import React, { useEffect, useState } from 'react'
import {
  Link,
  useRouteMatch
} from 'react-router-dom'
import './BudgetList.css'
import apiClient from './api-client'

export default function BudgetList () {
  const [budgets, setBudgets] = useState([])
  const match = useRouteMatch()

  useEffect(() => {
    apiClient.getBudgets()
      .then(json => {
        setBudgets(json.budgets)
      })
  }, [setBudgets])

  return (
    <div className="BudgetList">
      <h1>Budgets</h1>
      <ul>
        {
          budgets.map((budget, idx) => {
            return (
              <li key={idx}>
                <Link to={`${match.url}/${budget.id}`}>{budget.name}</Link>
              </li>
            )
          })
        }
      </ul>
    </div>
  )
}
