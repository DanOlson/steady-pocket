# Steady Pocket

A budgeting application

## Development

### Running the API

Install cargo-binstall

```bash
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
```

Install cargo-watch

```bash
cargo binstall cargo-watch
```

Start the server (auto-reloading enabled)

```bash
cargo watch -x run
```

The app will run on port 8081

### Running the frontend

The client is a React app built with [Vite](https://vite.dev). For development
with hot reloading:

```bash
cd ./client
npm install
npm run dev
```

The dev server runs on port 3001 and proxies `/api` requests to the backend
on port 8081, so start the API first.

Alternatively, build the static bundle and let the API serve it:

```bash
cd ./client
npm run build
```

With the client app built, you can visit `http://localhost:8081/`

Run the frontend tests (Vitest) with:

```bash
cd ./client
npm test
```

## Build & Deployment

### Docker

Build the image:

```bash
docker build -t budget .
```

Tag and push to Docker Hub (replace `<sha>` with the short commit hash from `git
rev-parse --short HEAD`):

```bash
docker tag budget dd1n/budget:<sha>
docker tag budget dd1n/budget:latest
docker push dd1n/budget:<sha>
docker push dd1n/budget:latest
```
