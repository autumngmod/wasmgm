# wasmgm

`wasmgm` is a **WASM runtime for Garry's Mod**, allowing you to write game logic in any language that compiles to WebAssembly.

> [!IMPORTANT]
> **wasmgm** is still under active development.
> Expect bugs, missing features, and overall instability.

---

## Usage
Just drop ``.wasm`` files to ``garrysmod/data/wasmgm/*.wasm``

## ✨ Features

- [ ] **Cross-platform** — Write once, run anywhere
- [ ] **Cross-language** — Use any language that compiles to **WASM**
- [ ] **Fast** — WebAssembly is often faster than Lua
- [ ] **Sandboxed** — Runs code in a safe, isolated environment
- [ ] **GMod Integration** — Hook events, call Lua, interact with the game (WIP)
<!-- - [ ] **Hot-reload** — Reload WASM without restarting the server (WIP)-->

# Compiling
## Cloning
```shell
git clone https://github.com/autumngmod/wasmgm
cd wasmgm
```

## Building
```shell
cargo build --target <TARGET> # Select a target from the table below
```
<!-- https://raw.githubusercontent.com/WilliamVenner/gmod-rs/refs/heads/master/examples/my-first-binary-module/README.md -->
| Platform | Command | Description |
|:---:|:---:|:---:|
| `win32` | `cargo build --target i686-pc-windows-msvc` | Windows 32-bit<br>Use this if your server is running Windows and is on the `main` branch of Garry's Mod (this is the default branch.) |
| `win64` | `cargo build --target x86_64-pc-windows-msvc` | Windows 64-bit<br>Use this if your server is running Windows and is on the `x86-64` branch of Garry's Mod. |
| `linux` | `cargo build --target i686-unknown-linux-gnu` | Linux 32-bit<br>Use this if your server is running Linux and is on the `main` branch of Garry's Mod (this is the default branch.) |
| `linux64` | `cargo build --target x86_64-unknown-linux-gnu` |Linux 64-bit<br>Use this if your server is running Linux and is on the `x86-64` branch of Garry's Mod. |

* You can read some instructions for [build from here](https://wiki.facepunch.com/gmod/Creating_Binary_Modules:_Rust)