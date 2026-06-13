import React from 'react'
import { formatCurrency } from '../format'
import './BudgetMeter.css'

// The spend indicator: a ruled bar with a pace tick marking where spending
// "should" be today, assuming an even burn across the month. Amounts are in
// cents; formatting and status (ok / warn / over) are derived internally.

const WARN_THRESHOLD = 0.85

export default function BudgetMeter ({ spent = 0, budgeted = 0, label, hero = false }) {
  const ratio = budgeted > 0 ? spent / budgeted : (spent > 0 ? Infinity : 0)
  const status = ratio > 1 ? 'over' : ratio >= WARN_THRESHOLD ? 'warn' : 'ok'
  const fillPercent = Math.min(ratio, 1) * 100
  const overage = Math.max(spent - budgeted, 0)

  const today = new Date()
  const daysInMonth = new Date(today.getFullYear(), today.getMonth() + 1, 0).getDate()
  const pacePercent = (today.getDate() / daysInMonth) * 100

  const bar = (
    <div
      className="budget-meter-bar"
      role="meter"
      aria-label={label ? `${label} spending` : 'Spending'}
      aria-valuemin={0}
      aria-valuemax={budgeted / 100}
      aria-valuenow={Math.min(spent, budgeted) / 100}
      aria-valuetext={`${formatCurrency(spent / 100)} of ${formatCurrency(budgeted / 100)}`}
    >
      <div className="budget-meter-fill" style={{ width: `${fillPercent}%` }} />
      <div
        className="budget-meter-pace"
        style={{ left: `${pacePercent}%` }}
        title={`Day ${today.getDate()} of ${daysInMonth}`}
      />
    </div>
  )

  if (hero) {
    return (
      <div className={`budget-meter budget-meter-hero is-${status}`}>
        {label && <span className="budget-meter-eyebrow">{label}</span>}
        <div className="budget-meter-figure">{formatCurrency(spent / 100)}</div>
        <p className="budget-meter-of">
          of <span className="budget-meter-num">{formatCurrency(budgeted / 100)}</span> budgeted
          {overage > 0 && <span className="budget-meter-stamp">Over +{formatCurrency(overage / 100)}</span>}
        </p>
        {bar}
        <p className="budget-meter-pace-note">
          <span className="budget-meter-pace-key" /> Today — day {today.getDate()} of {daysInMonth}
        </p>
      </div>
    )
  }

  return (
    <div className={`budget-meter is-${status}`}>
      <div className="budget-meter-line">
        <span className="budget-meter-label">
          {label}
          {overage > 0 && <span className="budget-meter-stamp">Over +{formatCurrency(overage / 100)}</span>}
          {status === 'warn' && <span className="budget-meter-warn-badge">Close</span>}
        </span>
        <span className="budget-meter-figures">
          <span className="budget-meter-num budget-meter-spent">{formatCurrency(spent / 100)}</span>
          {' / '}
          <span className="budget-meter-num">{formatCurrency(budgeted / 100)}</span>
        </span>
      </div>
      {bar}
    </div>
  )
}
