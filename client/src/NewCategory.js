import React from 'react'
import { useParams } from 'react-router-dom'
import ExpenseCategoryForm from './ExpenseCategoryForm'
import apiClient from './api-client'
import './NewCategoryForm.css'

export default function NewCategory () {
  const { budgetId } = useParams()

  function handleSubmit (category) {
    return apiClient.createCategory(category)
  }

  return (
    <ExpenseCategoryForm
      budgetId={Number(budgetId)}
      headingText="New Category"
      onSubmit={handleSubmit}
    />
  )
}
