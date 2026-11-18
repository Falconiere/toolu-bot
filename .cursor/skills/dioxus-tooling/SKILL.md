---
name: dioxus-tooling
description: >-
  Run and configure the Dioxus 0.7 CLI: dx serve, dx bundle, Dioxus.toml, web,
  desktop, and mobile targets, Tailwind, and release size. Use whenever the
  user asks how to create, serve, bundle, test, or configure a Dioxus app, or
  mentions dx, wasm, webview, iOS, or Android.
---

# Dioxus 0.7 tooling

Source: [Guides](https://dioxuslabs.com/learn/0.7/guides/) plus the fullstack project-setup page. This checkout is already a workspace (`packages/ui`, `packages/web`, `packages/desktop`, `packages/mobile`, `packages/api`). Do not scaffold a second app over it. Serve from the platform crate you are changing.

## Install and serve

```sh
rustup target add wasm32-unknown-unknown
curl -sSL https://dioxus.dev/install.sh | bash
dx doctor
```

`cargo binstall dioxus-cli` and `cargo install dioxus-cli` are the other install paths. OpenSSL failures use the openssl crate's automatic-deps section.

| Goal | Command |
|---|---|
| Web dev server | `dx serve` or, once a `server` feature exists, `dx serve --web` |
| Desktop | `dx serve --desktop` |
| iOS simulator | `dx serve --ios` (simulator already booted) |
| Android emulator | `dx serve --android` (emulator already booted) |
| Experimental Rust hot patch | `dx serve --hotpatch` (tip crate only; press `r` for a full rebuild) |
| Workspace client and server bins | `dx serve @client --bin dog-app @server --bin pet-api` |
| Release web bundle | `dx bundle --web --release` |
| Static site | `dx bundle --web --ssg` |
| HTML to RSX | `dx translate --file index.html` or `dx translate --raw "<div>Hello</div>"` |

`dx serve` in the tutorial listens on `http://127.0.0.1:8080`. `--web` selects target `wasm32-unknown-unknown`, feature `web`, profile `wasm-dev`. `--desktop` selects the host, feature `desktop`, profile `desktop-dev`, and the tutorial says DX passes `--no-default-features --features desktop`. A crate that existed only inside `default` disappears on desktop. `--ios` selects `aarch64-apple-ios-sim` and feature `mobile`.

`dx translate` prints RSX to stdout unless you pass `--output`. `--component` emits `fn component() -> Element`.

`dx new` prompts for a template (Bare-bones, Jumpstart, Workspace). The published pages do not document a `--platform` flag. The Workspace template is "a cargo workspace with a crate per platform." It does not name `ui` / `web` / `api`. This repo already uses those names. `[application] sub_package` selects which member DX builds.

## Features

```toml
[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
server = ["dioxus/server"]
```

`platforms/index.md` uses `default = []` instead of `default = ["web"]`. Gate platform code with `#[cfg(feature = "web")]`. Server-only dependencies stay optional. The old feature name `dioxus/axum` is `dioxus/server`. The old `ssr` feature name in `publishing.md` is the same split.

## Platforms

Web is WASM in the DOM. A custom `index.html` must contain `<div id="main">`. `web-sys` and `gloo` are web-only. Crates that need native timers or syscalls will not link.

Desktop is native Rust plus the system webview (wry). Browser APIs are not there, so WebGL and canvas are harder. Filesystem and sockets are. `document::eval` runs JavaScript in the webview. macOS needs no extra packages. Windows needs WebView2. Linux needs WebKitGTK; the getting-started page lists `libwebkit2gtk-4.1-dev` and `libxdo-dev` on Ubuntu. Hide the Windows console only on a ship build, or you lose logs:

```rust
#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]
```

`bundle` is an empty Cargo feature you pass at bundle time (`--features bundle`). It is not a `dx` flag.

Mobile uses the same webview. CSS is not identical on every webview. Android needs the SDK, NDK, and an emulator on `PATH`. iOS needs Xcode, and shipping needs signing, which Dioxus does not do. `dx bundle` builds desktop installers for the host OS only.

## Tailwind

If the crate root contains `tailwind.css` (`@import "tailwindcss";`), `dx` starts Tailwind and writes `assets/tailwind.css`. Load it with `document::Stylesheet { href: asset!("/assets/tailwind.css") }`. `tailwind_input` and `tailwind_output` in `Dioxus.toml` override the paths. Do not also run the npm watcher from the older utilities page.

## Verify

`cargo test` for units. RSX output can be compared with `pretty-assertions` and `dioxus-ssr`. The guide says there is no full hook-testing library. Web end-to-end is Playwright with `webServer.command` of `dx serve`, port `8080`, and a 10 minute timeout.

Logging comes from `dioxus::launch`: `Debug` in dev, `Info` in release. `tracing::debug!` is dev-only. Override before launch with `dioxus::logger::init(Level::INFO)`. The 0.6 name `dioxus::logger::initialize_default()` is the migration-guide spelling. `dioxus_logger::init` is gone. Android logs are `adb logcat`. iOS logs go to `oslog`.

Read [references/config-and-size.md](references/config-and-size.md) before editing `Dioxus.toml`, bundle identifiers, or a release profile.
