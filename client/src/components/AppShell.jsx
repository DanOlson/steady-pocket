import React from 'react'
import './AppShell.css'

// The screen scaffold: a mobile-first content column with an optional
// fixed action bar at the bottom (kept clear of home indicators). When an
// action is present the column gets bottom padding so content scrolls
// clear of it.

export default function AppShell ({ action, children }) {
  return (
    <div className={`app-shell${action ? ' has-action' : ''}`}>
      {children}
      {action && (
        <div className="app-shell-action-bar">
          <div className="app-shell-action-inner">{action}</div>
        </div>
      )}
    </div>
  )
}
