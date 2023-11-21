import React, { useEffect, useState } from 'react'
import { useParams } from 'react-router-dom'
import ExpenseCategoryForm from './ExpenseCategoryForm'
import apiClient from './api-client'

export default function EditExpenseCategory () {
  const { id, budgetId } = useParams()
  const [category, setCategory] = useState(null)

  useEffect(() => {
    apiClient.getCategory(id)
      .then(resp => setCategory(resp.category))
  }, [budgetId, id])

  function handleSubmit (category) {
    return apiClient.updateCategory({ ...category, id })
  }

  return (
    category && <ExpenseCategoryForm
      budgetId={budgetId}
      name={category.name}
      amount={category.amount / 100.0}
      headingText={`Edit Category`}
      onSubmit={handleSubmit}
    />
  )
}
