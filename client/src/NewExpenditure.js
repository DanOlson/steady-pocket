import React, { useState, useEffect } from 'react'
import { useLocation, useParams } from 'react-router-dom'
import ExpenditureForm from './ExpenditureForm'
import apiClient from './api-client'

function useQuery () {
  return new URLSearchParams(useLocation().search);
}

export default function NewExpenditure () {
  const query = useQuery()
  const categoryId = query.get('categoryId')
  const { budgetId } = useParams()
  const [category, setCategory] = useState({})

  useEffect(() => {
    apiClient.getCategory(categoryId)
      .then(json => {
        setCategory(json.category)
      })
  }, [budgetId, categoryId])

  function handleSubmit (expenditure) {
    return apiClient.createExpenditure({ ...expenditure, expenseCategoryId: category.id })
  }

  return (
    <ExpenditureForm
      budgetId={budgetId}
      category={category}
      onSubmit={handleSubmit}
    />
  )
}
