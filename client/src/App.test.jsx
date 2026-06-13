import React from 'react'
import { render, screen } from '@testing-library/react'
import { vi } from 'vitest'
import App from './App'

beforeEach(() => {
  vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
    ok: true,
    json: () => Promise.resolve({ budgets: [] })
  }))
})

afterEach(() => {
  vi.unstubAllGlobals()
})

test('renders the budgets nav link', async () => {
  render(<App />)
  expect(await screen.findByRole('link', { name: /budgets/i })).toBeInTheDocument()
})
