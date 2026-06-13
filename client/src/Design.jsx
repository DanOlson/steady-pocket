import React, { useState } from 'react'
import BudgetMeter from './components/BudgetMeter'
import Button from './components/Button'
import Card from './components/Card'
import EmptyState from './components/EmptyState'
import ListRow from './components/ListRow'
import Sheet from './components/Sheet'
import { TextField, MoneyField, SelectField } from './components/Field'
import './Design.css'

// Internal design-system reference page (route: /design). Renders the
// semantic tokens so both schemes can be checked at a glance; grows into
// the component gallery in Phase 2.

const colorTokens = [
  '--color-bg',
  '--color-surface',
  '--color-text',
  '--color-text-muted',
  '--color-border',
  '--color-border-strong',
  '--color-accent',
  '--color-on-accent',
  '--color-spend-ok',
  '--color-spend-ok-soft',
  '--color-spend-warn',
  '--color-spend-warn-soft',
  '--color-spend-over',
  '--color-spend-over-soft',
  '--color-focus'
]

const spaceTokens = ['--space-1', '--space-2', '--space-3', '--space-4', '--space-5', '--space-6', '--space-7']

const typeScale = [
  ['--text-2xl', 'Hero figure', '$912.20'],
  ['--text-xl', 'Screen title', 'Household'],
  ['--text-lg', 'Emphasized figure', '$86.20'],
  ['--text-md', 'Body', 'Pick a category to count this toward June.'],
  ['--text-sm', 'Caption', '51% spent, 40% of the month gone'],
  ['--text-xs', 'Eyebrow', 'NEEDS FILING']
]

export default function Design () {
  const [sheetOpen, setSheetOpen] = useState(false)

  return (
    <div className="design">
      <h1>Design tokens</h1>
      <p className="design-note">
        Semantic tokens from <code>styles/tokens.css</code>, rendered in the
        active color scheme. Flip the OS appearance setting to check the other
        scheme.
      </p>

      <h2>Color</h2>
      <div className="design-swatches">
        {colorTokens.map(token => (
          <div className="design-swatch" key={token}>
            <div className="design-swatch-chip" style={{ background: `var(${token})` }} />
            <code>{token}</code>
          </div>
        ))}
      </div>

      <h2>Type</h2>
      <div className="design-type">
        {typeScale.map(([token, role, sample]) => (
          <div className="design-type-row" key={token}>
            <code>{token}</code>
            <span className="design-type-role">{role}</span>
            <div className={`design-type-sample sample${token}`}>{sample}</div>
          </div>
        ))}
        <div className="design-type-row">
          <code>--font-mono</code>
          <span className="design-type-role">Figures (tabular)</span>
          <div className="design-type-sample design-figure">$1,800.00</div>
        </div>
      </div>

      <h2>Spacing</h2>
      <div className="design-spacing">
        {spaceTokens.map(token => (
          <div className="design-spacing-row" key={token}>
            <code>{token}</code>
            <div className="design-spacing-bar" style={{ width: `var(${token})` }} />
          </div>
        ))}
      </div>

      <h2>Status</h2>
      <div className="design-statuses">
        <span className="design-status is-ok">Under budget</span>
        <span className="design-status is-warn">Close to budget</span>
        <span className="design-status is-over">Over +$21.00</span>
      </div>

      <h2>BudgetMeter — hero</h2>
      <BudgetMeter hero spent={91220} budgeted={180000} label="Total spent" />

      <h2>BudgetMeter — compact</h2>
      <div className="design-stack">
        <BudgetMeter spent={28700} budgeted={62000} label="Groceries" />
        <BudgetMeter spent={13100} budgeted={15000} label="Fun money" />
        <BudgetMeter spent={17100} budgeted={15000} label="Dining out" />
        <BudgetMeter spent={4500} budgeted={0} label="No budget set" />
      </div>

      <h2>Buttons</h2>
      <div className="design-statuses">
        <Button variant="primary">Record expense</Button>
        <Button variant="secondary">Cancel</Button>
        <Button variant="quiet">Edit</Button>
        <Button variant="destructive">Delete category</Button>
        <Button variant="primary" size="sm">Save</Button>
      </div>

      <h2>Fields</h2>
      <TextField label="Description" name="description" defaultValue="Bulk groceries" onChange={() => {}} />
      <MoneyField label="Amount" name="amount" defaultCents={8620} onChange={() => {}} />
      <SelectField label="Category" name="category" defaultValue="" onChange={() => {}}>
        <option value="">Choose a category</option>
        <option value="1">Groceries</option>
        <option value="2">Dining out</option>
      </SelectField>

      <h2>List rows</h2>
      <ListRow
        to="/design"
        primary="Groceries"
        secondary="Costco · Bulk groceries"
        trailing="$86.20"
      />
      <ListRow
        primary="Dining out"
        secondary="June 9 · Taqueria"
        trailing="$24.50"
      />

      <h2>Card</h2>
      <Card>
        <BudgetMeter spent={28700} budgeted={62000} label="Groceries" />
      </Card>

      <h2>Empty state</h2>
      <EmptyState title="No expenses yet" message="Record an expense to start tracking June.">
        <Button variant="primary">Record expense</Button>
      </EmptyState>

      <h2>Sheet</h2>
      <Button variant="secondary" onClick={() => setSheetOpen(true)}>Open sheet</Button>
      <Sheet open={sheetOpen} title="Delete category" onClose={() => setSheetOpen(false)}>
        <p>Deleting <strong>Dining out</strong> removes its budget. Its expenses keep their history.</p>
        <Button variant="destructive" className="ui-btn-block" onClick={() => setSheetOpen(false)}>
          Delete category
        </Button>
        <Button variant="quiet" className="ui-btn-block" onClick={() => setSheetOpen(false)}>
          Keep it
        </Button>
      </Sheet>
    </div>
  )
}
