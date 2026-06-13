import React, { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import AppShell from './components/AppShell'
import BudgetMeter from './components/BudgetMeter'
import Button from './components/Button'
import Sheet from './components/Sheet'
import { TextField, MoneyField } from './components/Field'

// Shared by the new/edit expense screens. `amount` is in cents. Pass
// onDelete to offer deletion (edit screen) behind a confirmation sheet.

export default function ExpenditureForm (props) {
  const {
    budgetId,
    category,
    vendor,
    amount,
    description,
    heading,
    onSubmit,
    onDelete
  } = props

  const [expenditure, setExpenditure] = useState({ vendor, amount, description })
  const [confirmingDelete, setConfirmingDelete] = useState(false)
  const navigate = useNavigate()

  function handleSubmit (e) {
    e.preventDefault()
    if (expenditure.vendor && expenditure.amount && expenditure.description) {
      onSubmit(expenditure)
        .then(() => navigate(`/budgets/${budgetId}`))
    }
  }

  return (
    <AppShell>
      <h1 className="screen-title">{heading}</h1>
      {category && category.id && (
        <div className="form-context">
          <BudgetMeter
            label={category.name}
            spent={category.total_spend_to_date}
            budgeted={category.amount}
          />
        </div>
      )}
      <form onSubmit={handleSubmit}>
        <MoneyField
          label="Amount"
          name="amount"
          defaultCents={amount}
          onChange={cents => setExpenditure({ ...expenditure, amount: cents })}
        />
        <TextField
          label="Description"
          name="description"
          defaultValue={description}
          onChange={e => setExpenditure({ ...expenditure, description: e.target.value })}
        />
        <TextField
          label="Vendor"
          name="vendor"
          defaultValue={vendor}
          onChange={e => setExpenditure({ ...expenditure, vendor: e.target.value })}
        />
        <div className="form-actions">
          <Button variant="primary" className="ui-btn-block" type="submit">Save expense</Button>
          <Button variant="quiet" className="ui-btn-block" to={`/budgets/${budgetId}`}>Cancel</Button>
          {onDelete && (
            <Button
              variant="destructive"
              className="ui-btn-block"
              onClick={() => setConfirmingDelete(true)}
            >
              Delete expense
            </Button>
          )}
        </div>
      </form>
      {onDelete && (
        <Sheet
          open={confirmingDelete}
          title="Delete expense"
          onClose={() => setConfirmingDelete(false)}
        >
          <p>This removes the expense and its amount from the budget.</p>
          <div className="form-actions">
            <Button variant="destructive" className="ui-btn-block" onClick={onDelete}>
              Delete expense
            </Button>
            <Button variant="quiet" className="ui-btn-block" onClick={() => setConfirmingDelete(false)}>
              Keep it
            </Button>
          </div>
        </Sheet>
      )}
    </AppShell>
  )
}
