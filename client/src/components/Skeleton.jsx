import React from 'react'
import './Skeleton.css'

// Placeholder shown while a screen's data loads.
export default function Skeleton ({ lines = 4 }) {
  return (
    <div className="skeleton" aria-hidden="true">
      {Array.from({ length: lines }, (_, i) => (
        <div key={i} className="skeleton-line" />
      ))}
    </div>
  )
}
