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

### Building the frontend

```bash
cd ./client
npm run build
```

With the client app built, you can visit `http::localhost:8081/index.html`

## Build & Deployment

### Backend

To build the backend API app on MacOS, you need to have a cross compilation toolchain for arm-unknown-linux-gnueabihf instructions (targeting Raspberry Pi Zero). You can get one here:

https://thinkski.github.io/osx-arm-linux-toolchains/

With the toolchain available, and assuming it's in your `/Applications` directory, you can build the app as follows:

```bash
./script/backend/build.sh
```

Cargo needs to know the location of the linker from the toolchain. If you don't have the toolchain in `/Applications`, update the `.cargo/config` to correctly reference the linker.

Deploying the API can be done with this script:

```bash
./script/backend/deploy.sh
```

### Frontend

Building can be done with

```bash
./script/frontend/build.sh
```

Deployment is

```bash
./script/frontend/deploy.sh
```

### Unit file for running via Systemd

```
[Unit]
Description=Steady Pocket REST API for SmartMoney frontend
After=network.target

[Service]
Type=simple
User=pi
WorkingDirectory=/opt/smart-money
ExecStart=/opt/smart-money/steady-pocket
Environment="SERVER_ADDR=0.0.0.0:80"
Environment="DATABASE_URL=sqlite:./steady-pocket.db"
Restart=on-failure

[Install]
WantedBy=multi-user.target
```
