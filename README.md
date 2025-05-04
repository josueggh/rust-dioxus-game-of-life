# Game of Life — Rust + Dioxus
[![Rust](https://img.shields.io/badge/Language-Rust-orange?logo=rust)](https://www.rust-lang.org)

A Conway’s Game‑of‑Life implementation written in **Rust** with the
[Dioxus](https://dioxuslabs.com) UI framework.

![Demo](doc/demo.gif)

Live demo: https://dioxus-game-of-life.eusoj.dev/

---

## Prerequisites

* **Rust toolchain** – install via [rustup](https://rustup.rs).
* **Dioxus CLI**

  ```bash
  cargo install dioxus-cli
  ``` 

---

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/josueggh/rust-dioxus-game-of-life
cd rust-dioxus-game-of-life
```

### 2. Dev server (hot‑reload)

```bash
dx serve                # Web build 
# OR
dx serve --platform desktop   # Desktop build
```

### 3. Release builds

* **Web** (WASM + static assets):

  ```bash
  dx build --release
  ```
* **Desktop** (native binary + assets):

  ```bash
  dx build --platform desktop --release      
  ```

> Cargo resolves all dependencies automatically – nothing else to
> install.

---

## Configuration

Open **`src/main.rs`** and tweak the constants at the top of `App`:

```rust
let mut universe = use_signal(|| Universe::new(128, 128)); // width × height
let cell_size = 4;                                     // px per cell
const BASE_DELAY: u64 = 50;                            // ms between ticks
```

Change them, hit **save**, and `dx serve` hot‑reloads instantly.

---

## Common issues in Apple Silicon

If the cargo install `dioxus-cli` step fails with an _openssl-sys_ build
error, point Cargo to the correct directories and retry:
```bash
brew --prefix openssl@3
OPENSSL_LIB_DIR=$(brew --prefix openssl@3)/lib \
OPENSSL_INCLUDE_DIR=$(brew --prefix openssl@3)/include \
cargo install dioxus-cli
```

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

