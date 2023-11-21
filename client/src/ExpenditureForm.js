import React, { useState } from 'react'
import { Redirect } from 'react-router-dom'
import BudgetLink from './BudgetLink'
import ExpenseCategory from './ExpenseCategory'
import './Expenditure.css'

export default function ExpenditureForm (props) {
  const {
    budgetId,
    category,
    vendor,
    amount,
    description,
    onSubmit
  } = props

  const [expenditure, setExpenditure] = useState({ vendor, amount, description })
  const [success, setSuccess] = useState(false)

  function handleSubmit (e) {
    e.preventDefault()
    if (expenditure.vendor && expenditure.amount && expenditure.description) {
      onSubmit(expenditure)
        .then(() => setSuccess(true))
    }
  }

  function setAmount(e) {
    const amount = e.target.value
    setExpenditure({ ...expenditure, amount: amount * 100 })
  }

  function setDescription(e) {
    const description = e.target.value
    setExpenditure({ ...expenditure, description })
  }

  function setVendor(e) {
    const vendor = e.target.value
    setExpenditure({ ...expenditure, vendor })
  }

  if (success) {
    return <Redirect to={`/budgets/${budgetId}`} />
  }

  return (
    <div className="new-expenditure">
      <ExpenseCategory
        categoryName={category.name}
        amount={category.amount / 100.0}
        totalSpend={category.total_spend_to_date / 100.0}
      />
      <form className="new-expenditure-form" onSubmit={handleSubmit}>
        <div className="form-group row">
          <label className="col-form-label col-sm-2">Description</label>
          <input
            className="form-control col-sm-10"
            type="text"
            name="description"
            onChange={setDescription}
            defaultValue={expenditure.description}>
          </input>
        </div>

        <div className="form-group row">
          <label className="col-form-label col-sm-2">Vendor</label>
          <input
            className="form-control col-sm-10"
            type="text"
            name="vendor"
            onChange={setVendor}
            defaultValue={expenditure.vendor}>
          </input>
        </div>

        <div className="form-group row">
          <label className="col-form-label col-sm-2">Amount</label>
          <input
            className="form-control col-sm-10"
            type="number"
            step="any"
            name="amount"
            onChange={setAmount}
            defaultValue={expenditure.amount}>
          </input>
        </div>

        <div className="form-group row">
          <button className="btn btn-primary offset-sm-2" type="submit">Submit</button>
          <BudgetLink className="btn btn-outline-secondary" budgetId={budgetId}>Cancel</BudgetLink>
        </div>
      </form>
    </div>
  )
}
