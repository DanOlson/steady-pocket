import React, { useState, useEffect } from 'react'
import { useParams } from 'react-router-dom'
import apiClient from './api-client'
import ExpenseCategory from './ExpenseCategory'
import Expenditures from './Expenditures'
import NewExpenditureLink from './NewExpenditureLink'
import EditCategoryLink from './EditCategoryLink'
import DeleteCategoryLink from './DeleteCategoryLink'
import BudgetLink from './BudgetLink'
import './Expenditure.css'

export default function ExpenseCategoryDetail () {
  const { id, budgetId } = useParams()
  const [category, setCategory] = useState({})
  const [expenditures, setExpenditures] = useState([])

  useEffect(fetchData, [id])

  function fetchData () {
    apiClient.getCategory(id)
      .then(json => {
        setCategory(json.category)
        setExpenditures(json.expenditures)
      })
  }

  return (
    <div className="expense-category-detail">
      <ExpenseCategory
        totalSpend={category.total_spend_to_date / 100.0}
        amount={category.amount / 100.0}
        categoryName={category.name}
      >
        <Expenditures
          budgetId={budgetId}
          expenditures={expenditures}
          onDelete={fetchData}
        />
        <NewExpenditureLink
          className="btn btn-primary"
          categoryId={category.id}
          budgetId={budgetId}
        >
          Add
        </NewExpenditureLink>
        <BudgetLink className="btn btn-outline-secondary" budgetId={budgetId}>Back</BudgetLink>
        <EditCategoryLink
          className="btn btn-outline-info"
          budgetId={budgetId}
          categoryId={id}>Edit
        </EditCategoryLink>
        <DeleteCategoryLink budgetId={budgetId} category={category} />
      </ExpenseCategory>
    </div>
  )
}
