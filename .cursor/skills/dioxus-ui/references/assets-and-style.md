# Styles, assets, hot reload, escape hatches

From https://dioxuslabs.com/learn/0.7/essentials/ui/ (styling, assets, hotreload, escape).

## Inline style

One string, or one snake_case attribute per property. Values may be expressions.

```rust
div {
    background_color: if dark() { "black" } else { "white" },
    font_size: "16px",
    "Text"
}
```

Prefer CSS variables (`var(--color-primary)`) over formatting colors in Rust.

## Stylesheets

```rust
static MAIN_CSS: Asset = asset!("/assets/main.css");

rsx! { document::Stylesheet { href: MAIN_CSS } }
```

`document::Stylesheet` is marked pre-loadable under SSR. A plain `link { href: asset!(...) }` works and is not. SCSS is an asset too: `asset!("/assets/styles.scss")`.

## Tailwind (0.7)

`dx` downloads and runs Tailwind when the project root has `tailwind.css`:

```css
@import "tailwindcss";
@source "./src/**/*.{rs,html,css}";
```

Output defaults to `assets/tailwind.css`. Override with `tailwind_input` and `tailwind_output` in `Dioxus.toml`. Theme tokens go in `@theme { ... }` in the same root file.

The older guide that tells you to run `npx @tailwindcss/cli` beside `dx serve` is the pre-0.7 flow. Do not add that process when the root `tailwind.css` is present.

VS Code class detection: `"tailwindCSS.experimental.classRegex": ["class: \"(.*)\""]` and `"tailwindCSS.includeLanguages": { "rust": "html" }`.

## asset!

`asset!` is `const` and returns `Asset`. It does not embed bytes. It stores metadata in a `dx-assets` link section, and `dx serve` / `dx build` copies and hashes the file (`/assets/ferrous_wave-dxhx13xj2j.png`). Disable the hash with `asset!(path, AssetOptions::builder().with_hash_suffix(false))`.

Image options use one builder. `ImageAssetOptions::new()` is the 0.6 API.

```rust
asset!("/assets/image.png", AssetOptions::image().with_size(ImageSize::Manual { width: 52, height: 52 }))
asset!("/assets/image.png", AssetOptions::image().with_format(ImageFormat::Avif))
```

`/public` is copied and merged into the output `public` directory (`robots.txt`). It can overlay `public/assets`.

`asset!` does not bundle a directory as one blob. Join a filename onto the hashed folder: `format!("{}/logging.js", asset!("/assets/posthog-js"))`.

A library exports `pub static GREEN_STYLES: Asset = asset!("/assets/red.css");`. The app must reference that static or the file is not shipped.

Read bytes with `read_asset_bytes` in `asset_resolver` (web: fetch, native: the bundle). `asset_path` is only valid where the files are on a real filesystem (not the browser, not Android).

## Hot reload

RSX hot reload (no Rust recompile) can add, remove, and edit elements, edit string attributes (`class`, `id`, `title`, `style`), move an existing format expression, and change prop literals. It applies inside `if`, `for`, and component children.

It does not pick up new variables, logic outside `rsx!`, component signature changes, imports, or attribute expressions that call functions. Press `r` in the DX TUI for a full rebuild.

`dx serve --hotpatch` patches most Rust in the tip crate via Subsecond. New global destructors do not run, renamed globals are new globals, static initializer changes are not observed, and workspace dependency crates are not patched. RSX hot reload does cross a workspace. CSS, SCSS, images referenced with `asset!`, and the Tailwind output hot reload.

## Escape hatches

`dangerous_inner_html` sets the element's content and overwrites its other attributes and children. Sanitize untrusted HTML, or pass it as a text node so tags are escaped.

```rust
svg { dangerous_inner_html: r#"<path d="M256 352 128 160h256z" />"# }
```

A tag containing `-` is an untyped web component. Quote every attribute. Wrap it in a `#[component]` so callers pass typed props.

`document::eval` runs JavaScript. The documented receive loop is `let mut eval = document::eval(r#"... dioxus.send(i); ..."#);` then `eval.recv().await`. `eval.send` takes `Serialize`; `recv` returns `DeserializeOwned`.

`web_sys` is web-only. It compiles on desktop and mobile and panics when called. Gate it with `#[cfg(feature = "web")]`.

`onmounted` fires after the element is mounted. The 0.7 `Link` props include `onmounted: Option<EventHandler<MountedEvent>>`. DOM writes belong in `use_effect`, after render, so hydration can match.
