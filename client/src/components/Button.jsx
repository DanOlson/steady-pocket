import React from 'react'
import { Link } from 'react-router-dom'
import './Button.css'

// Variants: primary (the screen's main action), secondary (outlined),
// quiet (text-only), destructive. Pass `to` to render a router link.

export default function Button ({ variant = 'secondary', size, to, type = 'button', className, children, ...rest }) {
  const classes = ['ui-btn', `ui-btn-${variant}`, size === 'sm' && 'ui-btn-sm', className]
    .filter(Boolean)
    .join(' ')

  if (to) {
    return <Link className={classes} to={to} {...rest}>{children}</Link>
  }
  return <button className={classes} type={type} {...rest}>{children}</button>
}
