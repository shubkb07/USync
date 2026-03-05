# USync App (Rust + GTK)

USync is now implemented in **Rust** with:
- **CLI mode**
- **GTK4 GUI mode**
- **Clipboard history panel** (top of GUI)

## What to install (Ubuntu/Debian)

## Rust toolchain requirement

This project currently requires a modern Cargo/Rust toolchain (**Rust 1.92+ recommended**, and at least a Cargo version that supports lockfile v4 and edition2024 dependencies).

If you see errors like `lock file version 4` or `edition2024`, update Rust with:

```bash
rustup update
rustup default stable
```

```bash
sudo apt update
sudo apt install -y \
  build-essential \
  pkg-config \
  libgtk-4-dev \
  libxdo-dev \
  clang

# Install Rust toolchain (if you don't have it)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

## Run locally (development)

```bash
cargo run -- --name Alice
cargo run -- add 4 5
cargo run -- gui
```

## Build release binary

```bash
cargo build --release
./target/release/usync-app gui
```

## Build a `.deb` package

```bash
sudo apt-get install -y debhelper-compat dh-cargo cargo rustc

dpkg-buildpackage -us -uc -b
```

The generated `.deb` appears in parent directory (e.g. `../usync-app_0.1.0-1_all.deb`).

## Desktop launcher

After installing the `.deb`, open Ubuntu app menu and search for **USync Clipboard**.
