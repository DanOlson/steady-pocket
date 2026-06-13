import React from 'react'
import Gauge from './Gauge'
import { formatCurrency } from './format'

export default function ExpenseCategory ({ categoryName, amount, totalSpend, children }) {
  return (
    <div className="expense-category">
      <Gauge
        value={totalSpend}
        max={amount}
        label={categoryName}
        units={formatCurrency(amount)}
      />
      {children}
    </div>
  )
}
