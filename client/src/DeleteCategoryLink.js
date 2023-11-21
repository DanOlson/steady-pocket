import React, { useState } from 'react'
import { Redirect } from 'react-router-dom'
import apiClient from './api-client'

export default function DeleteCategoryLink ({ budgetId, category }) {
  const [success, setSuccess] = useState(false)

  function handleClick (e) {
    e.preventDefault()
    apiClient.deleteCategory(category.id)
      .then(() => setSuccess(true))
  }

  if (success) {
    return <Redirect to={`/budgets/${budgetId}`} />
  }

  return (
    <button className="btn btn-outline-danger" onClick={handleClick}>Delete</button>
  )
}
