# iced_baseview

![Test](https://github.com/BillyDM/iced_baseview/workflows/Rust/badge.svg)
[![License](https://img.shields.io/crates/l/iced_baseview.svg)](https://codeberg.org/BillyDM/iced_baseview/src/branch/main/LICENSE)

A [`baseview`](https://github.com/RustAudio/baseview) backend for [`Iced`](https://github.com/hecrj/iced). This also contains an adapter for [`NIH-plug`](https://github.com/robbert-vdh/nih-plug) (WIP).

<div align="center">
    <img src="screenshot.png">
</div>

The [main branch](https://codeberg.org/BillyDM/iced_baseview/src/branch/main) of this repository tracks the latest crates.io release of iced, while [iced_git  branch](https://codeberg.org/BillyDM/iced_baseview/src/branch/iced_git) (WIP) tracks its git repository.

## How to use with custom plugin framework

*TODO*

## How to use with NIH-plug

Add the following to your `Cargo.toml` (in place of `nih_plug_vizia`):

```toml
nih_plug_iced = { git = "https://codeberg.org/BillyDM/iced_baseview", branch = "main" }
# Important: `wgpu` is extremely spammy with its logs, so disable it by adding these crates:
log = { version = "0.4", features = ["max_level_info"] }
tracing = { version = "0.1.44", default-features = true, features = ["max_level_info"] }
```

See the `nih_plug_gain_iced` example for how to hook up `iced` to your plugin.

To build the `nih_plug_gain_iced` example, run:
```
cargo xtask bundle nih_plug_gain_iced --release
```

## Prerequisites

### Linux

Install dependencies, e.g.,

```sh
sudo apt-get install libx11-dev libxcursor-dev libxcb-dri2-0-dev libxcb-icccm4-dev libx11-xcb-dev mesa-common-dev libgl1-mesa-dev libglu1-mesa-dev
```
