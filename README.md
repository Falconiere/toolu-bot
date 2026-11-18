# Development

## Linting and formatting

- **Rust:** `cargo fmt` + `dx fmt` (rsx), 2-space indent. `cargo clippy` with the workspace lints in `Cargo.toml`: every warning is an error, including dead code, `unwrap`/`expect` and `panic!`.
- **No suppressions:** clippy rejects `#[allow]`, and `unwrap`/`expect` are `forbid` (no attribute can silence them). A `git grep` check rejects `#[expect]`, `#[ignore]` and `#[rustfmt::skip]`. Fix the cause instead.
- **File size:** at most 400 code lines per `.rs` file, counted by `tokei` (comments and blank lines don't count). Split into modules past that.
- **Function size:** at most 50 code lines per function body (comments and blank lines don't count). Clippy's `too_many_lines` checks plain functions; `ast-grep` with `lint/fn-body.yml` also covers `#[component]` functions, which the macro hides from clippy.
- **CSS, JSON, TOML:** `oxfmt` (config in `.oxfmtrc.json`; skips Markdown and the verbatim design-system `bundle.css`).

Run once per clone to format and lint on every commit (needs `lefthook`, `dx`, `tokei`, `ast-grep`, `jq` and Node):

```sh
lefthook install
```

CI runs the same checks (`.github/workflows/lint.yml`). Run them all by hand:

```sh
lefthook run pre-commit --all-files
```

## Workspace

Your new workspace contains a member crate for each of the web, desktop and mobile platforms, a `ui` crate for shared components and a `api` crate for shared backend logic:

```
your_project/
├─ README.md
├─ Cargo.toml
└─ packages/
   ├─ web/
   │  └─ ... # Web specific UI/logic
   ├─ desktop/
   │  └─ ... # Desktop specific UI/logic
   ├─ mobile/
   │  └─ ... # Mobile specific UI/logic
   ├─ api/
   │  └─ ... # All shared server logic
   └─  ui/
      └─ ... # Component shared between multiple platforms
```

## Platform crates

Each platform crate contains the entry point for the platform, and any assets, components and dependencies that are specific to that platform. For example, the desktop crate in the workspace looks something like this:

```
desktop/ # The desktop crate contains all platform specific UI, logic and dependencies for the desktop app
├─ assets/ # Assets used by the desktop app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app. It also defines the routes for the desktop platform
│  ├─ views/ # The views each route will render in the desktop version of the app
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

When you start developing with the workspace setup each of the platform crates will look almost identical. The UI starts out exactly the same on all platforms. However, as you continue developing your application, this setup makes it easy to let the views for each platform change independently.

## Shared UI crate

The workspace contains a `ui` crate with components that are shared between multiple platforms. You should put any UI elements you want to use in multiple platforms in this crate. You can also put some shared client side logic in this crate, but be careful to not pull in platform specific dependencies. The `ui` crate starts out something like this:

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ hero.rs # The Hero component that will be used in every platform
│  ├─ echo.rs # The shared echo component that communicates with the server
│  ├─ navbar.rs # The Navbar component that will be used in the layout of every platform's router
```

## Shared backend logic

The workspace contains a `api` crate with shared backend logic. This crate defines all of the shared server functions for all platforms. Server functions are async functions that expose a public API on the server. They can be called like a normal async function from the client. When you run `dx serve`, all of the server functions will be collected in the server build and hosted on a public API for the client to call. The `api` crate starts out something like this:

```
api/
├─ src/
│  ├─ lib.rs # Exports a server function that echos the input string
```

### Serving Your App

Navigate to the platform crate of your choice:
```bash
cd web
```

and serve:

```bash
dx serve
```

