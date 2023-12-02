# Steady Pocket

A budgeting application

## Development

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
