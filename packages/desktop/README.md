# Desktop

The desktop entry point. `src/main.rs` launches the app and renders `ui::Theme` and the shared UI. All UI lives in `packages/ui`; add only desktop-specific wiring and dependencies here.

Serve it from this folder with `dx serve`.

The desktop entry point disables always-on-top when the `desktop` feature is enabled, including during development.
