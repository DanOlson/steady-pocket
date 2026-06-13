import React from 'react'
import { useNavigate } from 'react-router-dom'
import apiClient from './api-client'

export default function DeleteCategoryLink ({ budgetId, category }) {
  const navigate = useNavigate()

  function handleClick (e) {
    e.preventDefault()
    apiClient.deleteCategory(category.id)
      .then(() => navigate(`/budgets/${budgetId}`))
  }

  return (
    <button className="btn btn-outline-danger" onClick={handleClick}>Delete</button>
  )
}
