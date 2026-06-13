const wholeDollars = new Intl.NumberFormat('en-US', {
  style: 'currency',
  currency: 'USD',
  minimumFractionDigits: 0,
  maximumFractionDigits: 0
})

const dollarsAndCents = new Intl.NumberFormat('en-US', {
  style: 'currency',
  currency: 'USD',
  minimumFractionDigits: 2,
  maximumFractionDigits: 2
})

// Whole amounts stay clean ($620); fractional amounts always get full
// cents ($86.20), never a dangling tenth ($86.2).
export function formatCurrency (dollars) {
  const formatter = Number.isInteger(dollars) ? wholeDollars : dollarsAndCents
  return formatter.format(dollars)
}
