import React from 'react'
import BudgetMeter from './components/BudgetMeter'

// amount and totalSpend are in cents.
export default function ExpenseCategory ({ categoryName, amount, totalSpend, children }) {
  return (
    <div className="expense-category">
      <BudgetMeter
        label={categoryName}
        spent={totalSpend}
        budgeted={amount}
      />
      {children}
    </div>
  )
}
