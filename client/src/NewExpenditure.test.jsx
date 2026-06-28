import React from 'react'
import { render, screen } from '@testing-library/react'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import { vi } from 'vitest'
import NewExpenditure from './NewExpenditure'

beforeEach(() => {
  vi.stubGlobal('fetch', vi.fn())
})

afterEach(() => {
  vi.unstubAllGlobals()
})

function renderNewExpenditure () {
  render(
    <MemoryRouter initialEntries={['/budgets/1/expenditures/new']}>
      <Routes>
        <Route path="/budgets/:budgetId/expenditures/new" element={<NewExpenditure />} />
      </Routes>
    </MemoryRouter>
  )
}

test('focuses the amount field when recording a new expense', () => {
  renderNewExpenditure()
  expect(screen.getByLabelText(/amount/i)).toHaveFocus()
})
