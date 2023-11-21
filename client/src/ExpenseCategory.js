import React from 'react'
import Gauge from './Gauge'
import { format } from 'd3-format'

export default function ExpenseCategory ({ categoryName, amount, totalSpend, children }) {
  return (
    <div className="expense-category">
      <Gauge
        value={totalSpend}
        max={amount}
        label={categoryName}
        units={format('$,')(amount)}
      />
      {children}
    </div>
  )
}
