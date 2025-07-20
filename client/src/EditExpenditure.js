import React, { useEffect, useState } from 'react'
import { useParams } from 'react-router-dom'
import ExpenditureForm from './ExpenditureForm'
import apiClient from './api-client'

export default function EditExpenditure () {
  const [expenditure, setExpenditure] = useState(null)
  const [category, setCategory] = useState(null)
  const { budgetId, id } = useParams()

  useEffect(() => {
    apiClient.getExpenditure(id)
      .then(resp => {
        setExpenditure(resp.expenditure)
        return resp.expenditure
      })
      .then(expenditure => {
        return apiClient.getCategory(expenditure.category_id)
      })
      .then(resp => setCategory(resp.category))
  }, [id])

  function handleSubmit (updated) {
    return apiClient.updateExpenditure({ ...updated, id: expenditure.id })
  }

  return category && (
    <div className="edit-expenditure">
      <ExpenditureForm
        budgetId={budgetId}
        category={category}
        amount={Math.round(expenditure.amount / 100.0)}
        vendor={expenditure.vendor}
        description={expenditure.description}
        onSubmit={handleSubmit}
      />
    </div>
  )
}
