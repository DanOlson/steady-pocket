import React from 'react'
import { render, screen } from '@testing-library/react'
import BudgetMeter from './BudgetMeter'

test('shows spent and budgeted amounts', () => {
  render(<BudgetMeter spent={28700} budgeted={62000} label="Groceries" />)
  expect(screen.getByText('$287')).toBeInTheDocument()
  expect(screen.getByText('$620')).toBeInTheDocument()
  expect(screen.getByRole('meter')).toHaveAttribute('aria-valuenow', '287')
})

test('stamps overspent budgets with the overage', () => {
  render(<BudgetMeter spent={17100} budgeted={15000} label="Dining out" />)
  expect(screen.getByText(/over \+\$21/i)).toBeInTheDocument()
})

test('flags spending against a zero budget as over, without NaN', () => {
  const { container } = render(<BudgetMeter spent={4500} budgeted={0} label="No budget" />)
  expect(screen.getByText(/over \+\$45/i)).toBeInTheDocument()
  expect(container.textContent).not.toMatch(/NaN/)
})

test('renders zeroes when nothing is set', () => {
  const { container } = render(<BudgetMeter label="Empty" />)
  expect(screen.getByRole('meter')).toHaveAttribute('aria-valuenow', '0')
  expect(container.textContent).not.toMatch(/NaN/)
})
