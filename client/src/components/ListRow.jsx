import React from 'react'
import { Link } from 'react-router-dom'
import './ListRow.css'

// A tappable register row. Pass `to` for navigation rows (adds a chevron),
// or onClick for action rows; otherwise renders a static row.

export default function ListRow ({ to, onClick, primary, secondary, trailing, children }) {
  const content = (
    <>
      <span className="list-row-main">
        <span className="list-row-primary">{primary}</span>
        {secondary && <span className="list-row-secondary">{secondary}</span>}
      </span>
      {trailing && <span className="list-row-trailing">{trailing}</span>}
      {to && <span className="list-row-chevron" aria-hidden="true">›</span>}
      {children}
    </>
  )

  if (to) {
    return <Link className="list-row" to={to}>{content}</Link>
  }
  if (onClick) {
    return <button className="list-row" type="button" onClick={onClick}>{content}</button>
  }
  return <div className="list-row">{content}</div>
}
