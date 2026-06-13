import React from 'react'
import Button from './Button'
import './ErrorState.css'

export default function ErrorState ({ onRetry }) {
  return (
    <div className="error-state" role="alert">
      <p className="error-state-title">Couldn't reach the server</p>
      <p className="error-state-message">
        Check that the server is running, then try again.
      </p>
      {onRetry && (
        <div className="error-state-action">
          <Button variant="secondary" onClick={onRetry}>Try again</Button>
        </div>
      )}
    </div>
  )
}
