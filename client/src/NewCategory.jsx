import React from 'react'
import { useParams } from 'react-router-dom'
import ExpenseCategoryForm from './ExpenseCategoryForm'
import apiClient from './api-client'

export default function NewCategory () {
  const { budgetId } = useParams()

  function handleSubmit (category) {
    return apiClient.createCategory(category)
  }

  return (
    <ExpenseCategoryForm
      budgetId={Number(budgetId)}
      headingText="New category"
      onSubmit={handleSubmit}
    />
  )
}
