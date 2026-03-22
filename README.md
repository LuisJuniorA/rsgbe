# RSGBE — Relatively Simple Game Boy Emulator

**RSGBE** is a modular Game Boy (DMG) emulation stack built with Rust. It is designed with a strict separation between its core logic and its frontend, making it highly portable.

The project currently focuses on **WebAssembly (WASM)** integration via `wasm-bindgen`, allowing the emulator to run at native speeds directly in the browser.

## 👾 Key Features

- **`rsgbe-core`**: A standalone, headless engine handling CPU (LR35902), PPU, and memory mapping.
- **WASM First**: Native support for `wasm-bindgen` to facilitate web-based frontend development.
- **Modular Architecture**: The core is decoupled from any I/O, allowing you to build your own interface (Web, Desktop, or CLI).
- **Performance**: Leverages Rust's memory safety and speed for accurate cycle-based emulation.

---

## 🏗 Project Structure

- `rsgbe-core/`: The heart of the emulator. Contains the CPU, Bus, APU, and PPU logic.
- `wasm/` (or your web folder): The bridge using `wasm-bindgen` to expose the core to JavaScript/TypeScript.

---

## 🚀 Getting Started (Web)

### Prerequisites

You will need the Rust toolchain and `wasm-pack`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack
```

### Build for the Web

To compile the project into a WebAssembly module:

```bash
wasm-pack build --target web
```

---

## 🛠 Roadmap

- [x] **CPU**: Core SM83 instruction set.
- [x] **WASM Bridge**: Basic bindings for web integration.
- [x] **PPU**: Background, Window, and Sprite rendering.
- [x] **Memory**: Implementation of common MBCs (MBC1, MBC2, MBC3, MBC5).
- [x] **APU**: Audio processing and web-audio output.
- [ ] **Desktop Frontend**: Future implementation.

---

## 🤝 Contributing

This project is a work in progress. Contributions are welcome\!

1. Fork the project.
2. Create your feature branch.
3. Open a Pull Request with a clear description of your changes.

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

---

_Developed with ❤️ by [LuisJuniorA](https://github.com/LuisJuniorA) & [David Maniliuc](https://github.com/David-Maniliuc)_
