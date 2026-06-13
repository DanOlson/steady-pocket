import React, { useEffect, useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import ExpenditureForm from './ExpenditureForm'
import apiClient from './api-client'

export default function EditExpenditure () {
  const [expenditure, setExpenditure] = useState(null)
  const [category, setCategory] = useState(null)
  const { budgetId, id } = useParams()
  const navigate = useNavigate()

  useEffect(() => {
    apiClient.getExpenditure(id)
      .then(resp => {
        setExpenditure(resp.expenditure)
        // Uncategorized expenses have no category to show context for
        if (resp.expenditure.category_id) {
          apiClient.getCategory(resp.expenditure.category_id)
            .then(catResp => setCategory(catResp.category))
        }
      })
  }, [id])

  function handleSubmit (updated) {
    return apiClient.updateExpenditure({ ...updated, id: expenditure.id })
  }

  function handleDelete () {
    apiClient.deleteExpenditure(expenditure.id)
      .then(() => navigate(`/budgets/${budgetId}`))
  }

  return expenditure && (
    <ExpenditureForm
      heading="Edit expense"
      budgetId={budgetId}
      category={category}
      amount={expenditure.amount}
      vendor={expenditure.vendor}
      description={expenditure.description}
      onSubmit={handleSubmit}
      onDelete={handleDelete}
    />
  )
}
