---
name: dioxus-migration
description: >-
  Upgrade Rust UI code to Dioxus 0.7 and reject removed APIs. Use whenever
  code contains cx, Scope, use_state, use_ref, use_future, Fermi, Atom,
  dioxus_lib, render!, dioxus/axum, ImageAssetOptions, or a Dioxus version
  older than 0.7, and whenever the user says migrate, upgrade, or "this used
  to work".
---

# Dioxus 0.7 migration

Sources: [To 0.7](https://dioxuslabs.com/learn/0.7/migration/to_07/), [To 0.6](https://dioxuslabs.com/learn/0.7/migration/to_06/), and the 0.5 hook/props/fermi pages. Apply this before the UI, state, router, or fullstack skill when the code is not already 0.7.

`to_07.md` is a short list. Most of the APIs people still paste (`cx`, `use_state`, Fermi) died in 0.4–0.6 and stay dead. Do not generate them. This repo is on `dioxus` 0.7.1.

## 0.6 to 0.7

| 0.6 | 0.7 |
|---|---|
| crate `dioxus_lib` / `use dioxus_lib::prelude::*` | `dioxus`, feature `lib` when you want the library without a renderer: `dioxus = { version = "0.7", default-features = false, features = ["lib"] }` |
| `ImageAssetOptions::new().with_size(...)` | `AssetOptions::image().with_size(...)`. `.with_format(ImageFormat::Avif)` stays. The 0.7 sample still imports `ImageAssetOptions`; the constructor is what changed |
| Form `onsubmit` with an empty handler, expecting no navigation | `onsubmit: move \|e\| e.prevent_default()`. Submit is allowed by default. Desktop still blocks page navigation itself |
| `ServerFnError` imported from `server_fn` | The Dioxus `ServerFnError`, which converts into `dioxus::Error` |
| Bare `#[server]` (URL-encoded form) | Bare `#[server]` is JSON. To keep form encoding, the prose says `protocol = Http<PostUrl, Json>` and the sample writes `Http<GetUrl, Json>`. Do not silently pick one |
| `#[server(protocol = Http<Json, Json>)]` | Bare `#[server]` |
| Unqualified `use_drop`, `Runtime`, `queue_effect`, `provide_root_context` | Still in the library. They left the prelude. Import them. The guide does not name the modules |
| Axum 0.7, wry 0.51, server_fn 0.6 | Axum 0.8, wry 0.52, server_fn 0.7 |

Custom renderers only: HTML listeners take `ListenerCallback<$data>` via `SuperInto`, not `EventHandler<Event<$data>>`.

The migration pages do not document a `--fullstack` CLI flag. Do not add one. Platform flags are `--web`, `--desktop`, `--ios`, `--android`.

## Dead APIs

When you see the left column, write the right column. Do not leave both.

| See | Write |
|---|---|
| `fn App(cx: Scope) -> Element` and `cx.render(rsx!(...))` / `render!` | `fn App() -> Element` and `rsx!`. Props are owned arguments |
| `cx.props.name` | the argument `name` |
| `use_state(cx, \|\| 0)`, `use_ref`, `UseState` | `use_signal` |
| `use_shared_state_provider` / `use_shared_state` | `use_context_provider` / `use_context` of a `Signal` |
| `write_silent` | `peek` |
| `use_future(cx, (dep,), \|dep\| async { })` | `use_resource` or `use_loader` with one closure. No dependency tuple |
| `use_effect` / `use_memo` with a dependency tuple | one closure that reads the signals it depends on |
| `Atom`, `AtomRef`, `use_init_atom_root`, `use_read`, `use_set`, `use_atom_ref` | `Signal::global`. `Memo::global` for a global memo (`T: PartialEq`) |
| `&'a str` or `Element<'a>` props, `Scope<'a>` | owned `String` / `Element`. The 0.5 page calls the reactive replacement "mapped signals" |
| `#[derive(Props, PartialEq)]` without `Clone` | also derive `Clone` |
| `#[props(into)]` on `String` | delete it. `String` already accepts formatted text |
| a prop whose name starts with `A`–`Z` | rename it. Rejected since 0.6 |
| `return None` from a component | `rsx! {}` or `VNode::empty()`. `Element` is a `Result`, not an `Option` |
| `prevent_default: "onclick"` | `onclick: move \|evt\| evt.prevent_default()` |
| `launch(App)` from the prelude | `dioxus::launch(App)` |
| `eval("...")` | `document::eval("...")` |
| `dioxus_logger::init(...)` | nothing, or `dioxus::logger::init(...)` before `dioxus::launch`. The 0.6 page says `dioxus::logger::initialize_default()` |
| feature `dioxus/axum` | `dioxus/server` |
| `fullstack::Config` | `LaunchBuilder::new().with_cfg(...).launch(app)`. Address is env `IP` and `PORT` |
| `register_server_fns` | `register_server_functions` |
| `.signal()` / `.memo()` on a global | `.resolve()` |
| `asset!(image("./assets/a.png").size(100, 100))` | `asset!("/assets/a.png", AssetOptions::image().with_size(...))`. Paths are absolute from the crate root and start with `/` |

`use_coroutine` takes `impl FnMut`, not `FnOnce`. `ToRouteSegments` takes `&self` (implement it for `T`, not `&T`). With the `web` feature, every `Routable` field appears in `#[route(...)]`.

## While you edit

The current anti-pattern page is not a rename, and it still applies after the upgrade: stable list keys, no `Mutex` in props, no writes during render, split large state, no non-deterministic work in the component body, and `PartialEq` returns `false` when the child should update.

Read [references/zero-six.md](references/zero-six.md) when the code uses eval, hot reload, the old Axum serve builder, or generational-box.
