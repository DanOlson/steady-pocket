import React from 'react'
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
    </div>
  )
}
