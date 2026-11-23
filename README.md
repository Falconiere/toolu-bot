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

Git ignores local toolu scratch directories at `.toolu/tmp/` and `.toolu/.toolu/tmp/`.

## Workspace

The workspace has a crate for each platform (web, desktop, mobile), a `ui` crate that holds **all** UI, and an `api` crate for server functions:

```
toolu-bot/
├─ PRODUCT.md      # product context (users, platforms, principles)
├─ Cargo.toml      # workspace + lint rules
└─ packages/
   ├─ web/         # entry point only
   ├─ desktop/     # entry point only
   ├─ mobile/      # entry point only
   ├─ api/         # shared server functions
   └─ ui/          # every component, style, font and screen
```

## Platform crates

Each platform crate is a thin entry point: its `src/main.rs` launches the app and renders the shared UI from `ui`. Add only platform wiring here (window config, platform-only dependencies), never UI: web, desktop and mobile ship the same interface, desktop-led with full parity.

## Shared UI crate

`ui` holds the TooluBot design system and every screen built on it:

```
ui/
├─ assets/   # tokens and component CSS, fonts, the mark
└─ src/
   ├─ lib.rs     # public exports
   ├─ theme.rs   # Theme: loads tokens, styles and fonts; render once at the root
   └─ ...        # primitives, icons and the AppShell screen
```

## Shared backend logic

The workspace contains a `api` crate with shared backend logic. This crate defines all of the shared server functions for all platforms. Server functions are async functions that expose a public API on the server. They can be called like a normal async function from the client. When you run `dx serve`, all of the server functions will be collected in the server build and hosted on a public API for the client to call. No app uses it yet; `src/lib.rs` holds the starter echo function.

### Serving Your App

Navigate to the platform crate of your choice:
```bash
cd web
```

and serve:

```bash
dx serve
```

