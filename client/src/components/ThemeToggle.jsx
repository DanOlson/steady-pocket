import React, { useState } from 'react'
import { THEME_MODES, storedTheme, applyTheme } from '../theme'
import './ThemeToggle.css'

// Cycles auto -> light -> dark. See src/theme.js for the semantics.

export default function ThemeToggle () {
  const [mode, setMode] = useState(storedTheme)

  function cycle () {
    const next = THEME_MODES[(THEME_MODES.indexOf(mode) + 1) % THEME_MODES.length]
    setMode(next)
    applyTheme(next)
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
