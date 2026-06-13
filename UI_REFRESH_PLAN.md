# Steady Pocket UI Refresh Plan

A phased plan to take the client from its current state (CRA + Bootstrap 4 CDN +
ad-hoc CSS) to a modern, mobile-first design system with OS-following light/dark
themes — structured so that the foundational work is design-agnostic and a
specific visual direction can slot in at the end with minimal friction.

## Where we are

- **Toolchain:** create-react-app 3.4.1 (deprecated, struggles on modern Node),
  React 16.13, react-router-dom v5.
- **Styling:** Bootstrap 4 loaded from a CDN in `index.html` (breaks when the
  Pi-hosted app is used without internet), six per-component CSS files with no
  shared vocabulary, and hardcoded hex colors + inline styles inside `Gauge.js`.
- **Layout:** desktop-era patterns — a wide table for uncategorized
  expenditures, `text-align: center` everywhere, Bootstrap form rows with
  side-by-side labels. None of it is designed for a phone, which is the primary
  device.
- **Theming:** none. Colors are scattered literals; no dark mode.
- **Identity drift:** the repo is "Steady Pocket," the page title is "Smart
  Money," and the PWA manifest still says "Create React App Sample."
- **Spend indicator:** `Gauge.js` — a d3-built semicircular gauge used on the
  budget summary, each category card, and the category detail/form pages. The
  d3 packages (`d3-shape`, `d3-scale`, `d3-format`) exist only to support it.
- **Deployment:** the Rust backend serves `client/build` statically;
  `script/frontend/build.sh` tars that directory for the Pi Zero. Whatever we
  do must keep emitting a static bundle at `client/build` (or the scripts get a
  one-line update).

## Where we're going

Three load-bearing ideas, in dependency order:

1. **Semantic design tokens** (CSS custom properties) are the seam between
   foundation and visual design. All components consume tokens like
   `--color-surface`, `--space-4`, `--radius-card`; a "design" is then just a
   token sheet plus a handful of component-level decisions. Light/dark is a
   token swap, and a future redesign is too.
2. **A small owned component library** replaces Bootstrap. ~10 components cover
   this app entirely. Each is styled exclusively through tokens.
3. **`BudgetMeter` as an abstraction, not a gauge.** The information being
   conveyed is `{spent, budgeted, label}` plus a derived status
   (under / approaching / over budget). The component owns that contract; the
   visual treatment (gauge, ring, segmented bar…) is an internal detail we can
   swap during prototype review without touching any call site.

---

## Phase 0 — Toolchain modernization (no visual change) ✅ (done 2026-06-12)

Get off deprecated tooling first so everything after is built on current ground.

- Migrate `client/` from create-react-app to **Vite**. Configure
  `build.outDir: 'build'` and a dev-server proxy for `/api` → the Rust backend
  (port 8081), so `script/frontend/build.sh`, `deploy.sh`, and the backend's
  static-file handling all keep working unchanged.
- Upgrade **React 16 → 18** (`createRoot`) and **react-router-dom v5 → v6**
  (`Switch`→`Routes`, `Redirect`→`useNavigate`/`Navigate`). Mechanical given
  the app's size.
- Move the test setup from react-scripts/Jest to **Vitest** (one test file
  exists today).
- Delete the unregistered CRA `serviceWorker.js`; fix the dead `baseUrl`
  constant in `api-client.js` while in there.
- **Remove the Bootstrap CDN link.** Temporarily vendor a minimal reset so the
  app stays usable until Phase 2 replaces Bootstrap classes for real.
- Drop `d3-format` in favor of `Intl.NumberFormat` (a `formatCurrency` util);
  `d3-shape`/`d3-scale` go away with the old gauge in Phase 2.

**Exit criteria:** app builds with Vite, deploys to the Pi via the existing
scripts, looks roughly the same, all routes work.

## Phase 1 — Design tokens and theming ✅ (done 2026-06-12)

The foundational layer everything else consumes. Lives in
`client/src/styles/tokens.css` (+ `base.css` for the reset and element
defaults).

- **Two-tier tokens:** primitive scales (raw palette ramps, a modular type
  scale, a 4px-based spacing scale, radii, shadows/elevation) and **semantic
  tokens** that components actually use: `--color-bg`, `--color-surface`,
  `--color-surface-raised`, `--color-text`, `--color-text-muted`,
  `--color-accent`, `--color-border`, and budget-status colors
  `--color-spend-ok` / `--color-spend-warn` / `--color-spend-over`.
- **OS-following dark mode:**
  - `color-scheme: light dark` on `:root` so native form controls, scrollbars,
    and keyboards follow.
  - Semantic tokens defined for light, overridden under
    `@media (prefers-color-scheme: dark)`.
  - Structure it through a `[data-theme]` attribute indirection so a manual
    light/dark/auto toggle can be added later without reworking anything
    (auto = no attribute, media query decides).
  - `<meta name="theme-color">` entries for both schemes so the browser chrome
    matches.
- **Typography:** pick a body face and a tabular-numerals-capable numeric
  treatment (money is the star of every screen). If a webfont is chosen,
  self-host it — this app must work without internet. The specific faces are a
  design-direction decision deferred to the prototype review; the type *scale*
  and roles are fixed now.
- **Identity cleanup:** settle the name (assume **Steady Pocket** unless you
  say otherwise), fix `<title>`, rewrite `manifest.json` (name, theme/background
  colors per scheme), `viewport-fit=cover` + safe-area-inset padding for
  notched phones.

**Exit criteria:** a tokens demo page renders all semantic tokens in both
schemes; flipping the OS setting flips the app.

## Phase 2 — Component library ✅ (done 2026-06-12; the bootstrap shim
survives until the Phase 3 screen rebuilds remove the last `.btn`/`.table`
usage)

Replace Bootstrap classes and per-page CSS with ~10 owned components, each
token-driven and mobile-first (≥44px touch targets throughout):

| Component | Replaces / serves |
|---|---|
| `AppShell` | header/nav; sticky top bar, content column, bottom action area |
| `Button` (primary/secondary/quiet/destructive) | all `btn btn-*` usage |
| `Card` | category cards, list rows |
| `TextField`, `MoneyField`, `SelectField` | Bootstrap form groups; `MoneyField` uses `inputmode="decimal"` and handles the cents conversion currently duplicated in forms |
| `ListRow` | budget list, expenditure lists, uncategorized items (tables die here) |
| `Sheet` (bottom sheet/dialog) | delete confirmations, quick category assignment |
| `EmptyState` | empty budgets/categories/expenditures |
| `BudgetMeter` | `Gauge.js` |

**`BudgetMeter`** is the one to get right:

- Props: `{ spent, budgeted, label?, size? }` — values in cents, formatting
  internal. Status derived internally (e.g. warn ≥ 85%, over > 100%) and
  expressed through the status tokens.
- Must handle today's gauge's failure modes: overspend (clamped and invisible
  today) gets an explicit over-budget treatment; `budgeted = 0` doesn't NaN.
- Two sizes: `hero` (budget summary) and `compact` (category cards/rows).
- Built as plain SVG/CSS — no d3 needed for one arc or bar.
- The render internals are deliberately swappable; this is where prototype
  feedback lands.

Accessibility baseline while building: visible focus rings (token-driven),
status conveyed by more than color (icon/text, not just red vs green),
`prefers-reduced-motion` respected by any transitions.

**Exit criteria:** component gallery page showing everything in both schemes on
a phone-width viewport; Bootstrap reset shim deleted; d3 dependencies removed.

## Phase 3 — Screen-by-screen rebuild ✅ (done 2026-06-12)

Recompose each route from the library. Order chosen so the highest-traffic
screens land first:

1. **Budget overview** (`/budgets/:id`) — the home screen in practice. Hero
   `BudgetMeter` for total spend; category list as compact cards/rows with
   inline meters; persistent bottom-anchored **"Add expense"** action (the #1
   task on a phone) opening a `Sheet` or the form route; **uncategorized
   triage** redesigned from a 5-column table to a card stack — one card per
   expenditure with a category picker that saves on selection.
2. **Expenditure form** (new/edit) — amount-first entry with `MoneyField` and
   numeric keypad, stacked labels, full-width submit in thumb reach.
3. **Category detail** — meter header, expenditure `ListRow`s, edit/delete
   moved into an overflow/secondary placement with `Sheet` confirmation
   (replacing the current row of five buttons).
4. **Budget list** — card list; mostly trivial once the library exists.
5. **Category form** (new/edit) — same form patterns.

Each screen is an independent PR-sized change; old and new screens can coexist
during the transition since they share routes and the API client.

**Exit criteria:** no Bootstrap class names, no per-page CSS files with
hardcoded colors, every screen comfortable one-handed on a 375px viewport in
both schemes.

## Phase 4 — Polish ✅ (done 2026-06-12; known gap: Sheet doesn't trap
focus — fine for this app's size, revisit if forms ever move into sheets)

- Loading skeletons (every screen currently flashes empty while fetching) and
  basic error states — `api-client.js` does no error handling today.
- Motion pass: meter fill-in on load, sheet transitions; all gated on
  `prefers-reduced-motion`.
- PWA finish: real icons, verify installed/standalone display on iOS/Android.
- Quick Lighthouse + keyboard-nav audit.

---

## Prototype checkpoints (your two reviews)

The plan is sequenced so your design feedback is consumed where it's cheap:

- **Review 1 — design direction. ✅ Done (2026-06-12).** Two prototypes were
  built in `prototypes/` and reviewed: **"Ledger" (`a-ledger.html`) won** over
  "Dusk" (`b-dusk.html`). The Ledger direction defines the Phase 1 tokens and
  the `BudgetMeter` treatment:
  - Check-register vernacular: IBM Plex Mono tabular figures + IBM Plex Sans,
    paper-white/green-ink light theme, deep green-black dark theme, red ink
    for over-budget ("in the red") with a rubber-stamp `OVER +$X` badge,
    amber for approaching budget.
  - `BudgetMeter` renders as a ruled horizontal bar with a **pace tick**
    marking where spending *should* be today (day N of the month), so the
    bar communicates pace, not just fill.
  - The prototype's token values and meter markup are the reference
    implementation; Phase 1/2 translate them into `tokens.css` and the React
    `BudgetMeter`.
- **Review 2 — after Phase 3 screen #1 (the real thing).** The rebuilt budget
  overview running against the real backend, on a phone. Course-corrections
  here propagate cheaply because the remaining screens reuse the same
  components.

## Sequencing and effort

| Phase | Scope | Relative effort |
|---|---|---|
| 0 | Vite/React 18/Router v6 migration | Medium — mechanical but touches everything |
| 1 | Tokens, theming, identity | Small–medium |
| — | Prototype review 1 | Two throwaway HTML files |
| 2 | Component library | Medium — the core build |
| 3 | Five screens | Medium — spread across small PRs |
| — | Prototype review 2 | Checkpoint, not new work |
| 4 | Polish | Small |

## Risks / notes

- **Pi Zero deployment:** the only hard constraint is static files in
  `client/build`. Vite output is smaller than CRA's, and dropping
  Bootstrap + d3 shrinks it further — strictly better for the Pi.
- **React 18/Router v6 migration** is the riskiest single step, but the app is
  ~20 small files; doing it before any visual work isolates the risk.
- **Out of scope:** backend/API changes. The existing JSON contract is
  sufficient for everything above.
