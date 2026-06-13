import React from 'react'
import './Card.css'

export default function Card ({ className, children }) {
  return (
    <div className={['ui-card', className].filter(Boolean).join(' ')}>
      {children}
    </div>
  )
}
