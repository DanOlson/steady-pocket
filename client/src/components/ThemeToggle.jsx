import React, { useState } from 'react'
import './ThemeToggle.css'

// Cycles auto -> light -> dark. "Auto" follows the OS setting (no
// data-theme attribute); explicit choices are persisted and applied to
// <html> before first paint by the snippet in index.html.

const MODES = ['auto', 'light', 'dark']
const STORAGE_KEY = 'steady-pocket-theme'

export default function ThemeToggle () {
  const [mode, setMode] = useState(() => {
    const stored = localStorage.getItem(STORAGE_KEY)
    return MODES.includes(stored) ? stored : 'auto'
  })

  function cycle () {
    const next = MODES[(MODES.indexOf(mode) + 1) % MODES.length]
    setMode(next)
    if (next === 'auto') {
      localStorage.removeItem(STORAGE_KEY)
      document.documentElement.removeAttribute('data-theme')
    } else {
      localStorage.setItem(STORAGE_KEY, next)
      document.documentElement.setAttribute('data-theme', next)
    }
  }

  return (
    <button
      className="theme-toggle"
      type="button"
      onClick={cycle}
      aria-label={`Appearance: ${mode}. Tap to change.`}
    >
      {mode}
    </button>
  )
}
