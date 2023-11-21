import React from 'react'
import apiClient from './api-client'

export default function DeleteExpenditureLink ({ expenditure, onDelete }) {
  function handleClick (e) {
    e.preventDefault()
    apiClient.deleteExpenditure(expenditure.id)
      .then(onDelete)
  }

  return (
    <button
      className="btn btn-danger btn-sm"
      onClick={handleClick}>x
    </button>
  )
}
