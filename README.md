# RSGBE

**RSGBE** (Relatively Simple Game Boy Emulator) is a high-performance, lightweight Game Boy (DMG) emulation stack written in Rust.

The project is architected with a strict separation between its core logic and frontend implementations. At its heart lies `rsgbe-core`, a headless engine responsible for CPU cycles, PPU rendering, and memory management, making it easily embeddable into various platforms—from native desktop applications to web-based environments via WebAssembly (WASM).

## 🚀 Key Features

* **Modular Design**: A standalone core (`rsgbe-core`) that remains agnostic of the rendering backend.
* **WASM Ready**: Designed to be compiled for the web, allowing Game Boy emulation directly in the browser.
* **Platform Portable**: Can be wrapped in any language or framework (C, Python, JavaScript) for custom implementations.
* **Memory Safety**: Built with Rust's strict safety guarantees to ensure a crash-free emulation experience.

---

## 🏗 Architecture

The emulator is split into several logical components:

| Component | Description |
| --- | --- |
| **CPU** | A Sharp LR35902 implementation with accurate instruction timings and flag handling. |
| **Bus** | A centralized memory controller managing ROM, Work RAM (WRAM), and High RAM (HRAM). |
| **Registers** | Efficient 8-bit and 16-bit register management with bit-flag utilities. |
| **PPU (WIP)** | Headless pixel processing unit for tile-based rendering. |

---

## 🛠 Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/) (latest stable)
* `cargo`

### Compilation

To build the core library:

```bash
cargo build --release

```

---

## 📈 Current Status (Roadmap)

* [x] Base CPU instruction set (WIP)
* [x] Memory mapping (ROM, WRAM, HRAM)
* [ ] PPU / Graphics Rendering
* [ ] Audio (APU) support
* [ ] MBC (Memory Bank Controller) support
* [ ] WASM Frontend wrapper

---

## ⚖️ License

This project is licensed under the **MIT License**. See the `LICENSE` file for details.
