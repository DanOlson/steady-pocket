// Theme switching: 'auto' follows the OS (no data-theme attribute, the
// per-scheme theme-color metas stand as written in index.html); 'light' or
// 'dark' force a scheme and pin both metas so browser chrome matches.
// The inline script in index.html mirrors applyTheme for first paint —
// keep the two in sync.

export const THEME_MODES = ['auto', 'light', 'dark']
export const STORAGE_KEY = 'steady-pocket-theme'

const THEME_COLORS = {
  light: '#FBFBF7',
  dark: '#121613'
}

export function storedTheme () {
  const stored = localStorage.getItem(STORAGE_KEY)
  return THEME_MODES.includes(stored) ? stored : 'auto'
}

export function applyTheme (mode) {
  if (mode === 'light' || mode === 'dark') {
    localStorage.setItem(STORAGE_KEY, mode)
    document.documentElement.setAttribute('data-theme', mode)
    setMetaColors(THEME_COLORS[mode], THEME_COLORS[mode])
  } else {
    localStorage.removeItem(STORAGE_KEY)
    document.documentElement.removeAttribute('data-theme')
    setMetaColors(THEME_COLORS.light, THEME_COLORS.dark)
  }
}

function setMetaColors (lightColor, darkColor) {
  const light = document.querySelector('meta[name="theme-color"][media*="light"]')
  const dark = document.querySelector('meta[name="theme-color"][media*="dark"]')
  if (light) light.setAttribute('content', lightColor)
  if (dark) dark.setAttribute('content', darkColor)
}
