# Steady Pocket client

The frontend for Steady Pocket: React, built with [Vite](https://vite.dev),
tested with [Vitest](https://vitest.dev).

## Scripts

### `npm run dev`

Runs the Vite dev server with hot reloading at
[http://localhost:3001](http://localhost:3001). Requests to `/api` are
proxied to the backend API on port 8081 (see `vite.config.js`), so start
the API first — see the repo root README.

### `npm test`

Runs the test suite with Vitest in watch mode.

### `npm run build`

Builds the production bundle into `build/`, where the backend API serves
it as static files. `script/frontend/build.sh` at the repo root wraps this
for deployment.

### `npm run preview`

Serves the production bundle from `build/` locally.
