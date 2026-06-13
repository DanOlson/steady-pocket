import React from 'react'
import './EmptyState.css'

export default function EmptyState ({ title, message, children }) {
  return (
    <div className="empty-state">
      <p className="empty-state-title">{title}</p>
      {message && <p className="empty-state-message">{message}</p>}
      {children && <div className="empty-state-action">{children}</div>}
    </div>
  )
}
