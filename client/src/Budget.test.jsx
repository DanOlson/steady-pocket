import React from 'react'
import { render, screen } from '@testing-library/react'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import { vi } from 'vitest'
import Budget from './Budget'

const fixture = {
  budget: { id: 1, name: 'Household' },
  categories: [
    { id: 1, name: 'Groceries', amount: 62000, total_spend_to_date: 28700 },
    { id: 2, name: 'Dining out', amount: 15000, total_spend_to_date: 17100 }
  ],
  expenditures: [
    { id: 9, amount: 8620, description: 'Bulk groceries', vendor: 'Costco', category_id: null }
  ],
  summary: {
    budgeted_amount: 77000,
    total_spend_to_date: 54420,
    uncategorized_spend_to_date: 8620
  }
}

beforeEach(() => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
    json: () => Promise.resolve(fixture)
  }))
})

afterEach(() => {
  vi.unstubAllGlobals()
})

function renderBudget () {
  render(
    <MemoryRouter initialEntries={['/budgets/1']}>
      <Routes>
        <Route path="/budgets/:id" element={<Budget />} />
      </Routes>
    </MemoryRouter>
  )
}

test('renders the hero meter from the summary', async () => {
  renderBudget()
  expect(await screen.findByText('Household')).toBeInTheDocument()
  expect(screen.getByText('$544.20')).toBeInTheDocument()
})

test('lists categories with their meters, flagging overspend', async () => {
  renderBudget()
  expect(await screen.findByRole('link', { name: /groceries/i })).toBeInTheDocument()
  expect(screen.getByText(/over \+\$21/i)).toBeInTheDocument()
})

test('offers filing for uncategorized expenditures', async () => {
  renderBudget()
  expect(await screen.findByText(/needs filing/i)).toBeInTheDocument()
  expect(screen.getByText(/costco/i)).toBeInTheDocument()
  expect(screen.getByRole('combobox')).toBeInTheDocument()
})
