# USync Super App (Rust + GTK)

USync is now a **super app shell** with multiple tools. Current module:
- **Clipboard Manager** (persistent history)
- **Clipboard Settings** (history size + polling interval)

## System requirements (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y \
  build-essential \
  pkg-config \
  libgtk-4-dev \
  libglib2.0-dev \
  libxdo-dev \
  clang
```

## Rust toolchain

This project uses rustup-managed toolchains and is pinned by `rust-toolchain.toml`.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup update
rustup default stable
```

## Run locally

```bash
cargo run -- gui
```

## Build release binary

```bash
cargo build --release
./target/release/usync-app gui
```

## Build `.deb` package (cargo-deb)

```bash
cargo install cargo-deb
cargo deb
```

Output package is created under `target/debian/`.

Install it:

```bash
sudo dpkg -i target/debian/usync-app_0.2.0_amd64.deb
```

After install, search app menu for **USync Super App**.

Package metadata now ships AppStream metainfo (`com.usync.app.metainfo.xml`) and a desktop id
(`com.usync.app.desktop`) so software stores can resolve icon/description before install.

## Notes on top-bar clipboard indicator

The current app provides a full clipboard module inside the GTK app and persists history to disk.
A GNOME top-bar clipboard indicator requires shell-extension style integration or tray/indicator service, which is planned as a separate module.
