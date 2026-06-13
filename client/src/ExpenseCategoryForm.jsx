import React, { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import AppShell from './components/AppShell'
import Button from './components/Button'
import { TextField, MoneyField } from './components/Field'

// Shared by the new/edit category screens. `amount` is in cents.

export default function ExpenseCategoryForm ({ budgetId, headingText, name, amount, onSubmit }) {
  const [category, setCategory] = useState({ name, amount, budgetId })
  const [saveFailed, setSaveFailed] = useState(false)
  const navigate = useNavigate()

  function handleSubmit (e) {
    e.preventDefault()
    onSubmit(category)
      .then(() => navigate(`/budgets/${budgetId}`))
      .catch(() => setSaveFailed(true))
  }

  return (
    <AppShell>
      <h1 className="screen-title">{headingText}</h1>
      <form onSubmit={handleSubmit}>
        <TextField
          label="Name"
          name="name"
          defaultValue={name}
          onChange={e => setCategory({ ...category, name: e.target.value })}
        />
        <MoneyField
          label="Monthly budget"
          name="amount"
          defaultCents={amount}
          onChange={cents => setCategory({ ...category, amount: cents })}
        />
        {saveFailed && (
          <p className="form-error" role="alert">Couldn't save. Check the server, then try again.</p>
        )}
        <div className="form-actions">
          <Button variant="primary" className="ui-btn-block" type="submit">Save category</Button>
          <Button variant="quiet" className="ui-btn-block" to={`/budgets/${budgetId}`}>Cancel</Button>
        </div>
      </form>
    </AppShell>
  )
}
