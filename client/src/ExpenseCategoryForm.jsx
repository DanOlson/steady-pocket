import React, { useState } from 'react'
import { Redirect } from 'react-router-dom'
import BudgetLink from './BudgetLink'

export default function ExpenseCategoryForm ({ budgetId, headingText, name, amount, onSubmit }) {
  const [success, setSuccess] = useState(false)
  const [category, setCategory] = useState({ name, amount, budgetId })

  function setName(e) {
    setCategory({
      ...category,
      name: e.target.value
    })
  }

  function setAmount(e) {
    setCategory({
      ...category,
      amount: e.target.value * 100.0
    })
  }

  function handleSubmit(e) {
    e.preventDefault()
    onSubmit(category)
      .then(() => setSuccess(true))
  }

  if (success) {
    return <Redirect to={`/budgets/${budgetId}`} />
  }

  return (
    <div className="new-category">
      <h1>{headingText}</h1>
      <form className="new-category-form" onSubmit={handleSubmit}>
        <div className="form-group row">
          <label className="col-form-label col-sm-2">Name</label>
          <input
            className="form-control col-sm-10"
            type="text"
            name="name"
            defaultValue={name}
            onChange={setName}
          />
        </div>

        <div className="form-group row">
          <label className="col-form-label col-sm-2">Amount</label>
          <input
            className="form-control col-sm-10"
            type="number"
            step="any"
            name="amount"
            defaultValue={amount}
            onChange={setAmount}
          />
        </div>

        <div className="form-group row">
          <button className="btn btn-primary offset-sm-2" type="submit">Submit</button>
          <BudgetLink className="btn btn-outline-secondary" budgetId={budgetId}>Cancel</BudgetLink>
        </div>
      </form>
    </div>
  )
}
