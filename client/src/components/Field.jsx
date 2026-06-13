import React from 'react'
import './Field.css'

// Form fields with stacked labels. MoneyField speaks dollars to the person
// and cents to the caller: onChange receives an integer cent amount (or
// null while the input isn't a number).

export function TextField ({ label, name, defaultValue, onChange, ...rest }) {
  return (
    <label className="field">
      <span className="field-label">{label}</span>
      <input
        className="field-input"
        type="text"
        name={name}
        defaultValue={defaultValue}
        onChange={onChange}
        {...rest}
      />
    </label>
  )
}

export function MoneyField ({ label, name, defaultCents, onChange, ...rest }) {
  function handleChange (e) {
    const dollars = parseFloat(e.target.value)
    onChange(Number.isFinite(dollars) ? Math.round(dollars * 100) : null)
  }

  return (
    <label className="field">
      <span className="field-label">{label}</span>
      <div className="field-money">
        <span className="field-money-symbol" aria-hidden="true">$</span>
        <input
          className="field-input field-input-money"
          type="text"
          inputMode="decimal"
          autoComplete="off"
          name={name}
          defaultValue={defaultCents != null ? defaultCents / 100 : ''}
          onChange={handleChange}
          {...rest}
        />
      </div>
    </label>
  )
}

export function SelectField ({ label, name, value, onChange, children, ...rest }) {
  return (
    <label className="field">
      <span className="field-label">{label}</span>
      <select
        className="field-input field-select"
        name={name}
        value={value}
        onChange={onChange}
        {...rest}
      >
        {children}
      </select>
    </label>
  )
}
