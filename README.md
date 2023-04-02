<img align="left" style="vertical-align: middle" width="120" height="120" alt="Template Screenshot" src="data/icons/app.svg">

# libhelium Rust Template

The quickest way to get started with libhelium & Rust.

###

[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

## 🚀 Getting Started

Welcome! Please see the various resources below. If you have any questions, our [Discord](https://discord.gg/BHNfGewTXX) is always open :)

- [The tauOS HIG](https://developers.tauos.co/docs/hig)
- [libhelium's Rust Documentation](https://docs.developers.tauos.co/rust/libhelium/index.html)

> **Note**
> A tutorial for using this template is currently in-progress. If you'd like to help us, please join our [Discord](https://discord.gg/BHNfGewTXX)!

## 🛠️ Dependencies

Please make sure you have these dependencies first before building.

```bash
gtk4
libhelium-1
meson
cargo
rustc
blueprint-compiler
```

## 🏗️ Building

Simply clone this repo, then:

```bash
meson setup _build --prefix=/usr
meson compile -C _build
sudo meson install -C _build --no-rebuild
```
