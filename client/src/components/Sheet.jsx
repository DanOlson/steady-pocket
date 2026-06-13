import React, { useEffect } from 'react'
import './Sheet.css'

// A bottom sheet for confirmations and quick pickers. Closes on backdrop
// tap or Escape; the caller owns the open state.

export default function Sheet ({ open, title, onClose, children }) {
  useEffect(() => {
    if (!open) return
    function onKeyDown (e) {
      if (e.key === 'Escape') onClose()
    }
    document.addEventListener('keydown', onKeyDown)
    return () => document.removeEventListener('keydown', onKeyDown)
  }, [open, onClose])

  if (!open) return null

  return (
    <div className="sheet-backdrop" onClick={onClose}>
      <div
        className="sheet"
        role="dialog"
        aria-modal="true"
        aria-label={title}
        onClick={e => e.stopPropagation()}
      >
        <div className="sheet-handle" aria-hidden="true" />
        {title && <h2 className="sheet-title">{title}</h2>}
        {children}
      </div>
    </div>
  )
}
