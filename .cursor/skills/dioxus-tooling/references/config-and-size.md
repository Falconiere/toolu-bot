# Dioxus.toml, bundles, and release size

From https://dioxuslabs.com/learn/0.7/guides/tools/configure/ and https://dioxuslabs.com/learn/0.7/guides/deploy/config/. "🔒" on a header in the configure page means the header is mandatory; the keys inside are not. Bundling is a subset of tauri-bundler. Do not copy Tauri keys that these two pages do not list.

## Application and web

`[application]`: `name`, `out_dir` (examples `"dist"` and `"docs"`), `asset_dir` (`"public"`), `sub_package`.

`[web.app]`: `title`, `base_path` (the GitHub Pages repo name, not a full URL).

`[web.watcher]`: `reload_html`, `watch_path`, `index_on_404`. GitHub Pages client routing needs `index.html` copied to `404.html`. `index_on_404 = true` is the dev-server equivalent.

`[web.resource]` and `[web.resource.dev]`: `style`, `script`. The dev table applies to `dx serve`, not to `dx serve --release`.

`[[web.proxy]]`: `backend`. Path and query are forwarded. Path rewriting is not supported.

`[[web.https]]`: `enabled`, `mkcert`, `key_path`, `cert_path`.

`[web]`: `pre_compress` (default on, brotli in release).

The wasm-opt section is titled `Web.wasm_opt` and shows `level` (`"z"`, `"s"`, `"1"`–`"4"`, default `"4"`) and `debug`. It never prints a `[web.wasm_opt]` header. The optimizing page still says wasm-opt is not in the CLI yet and shows a manual `wasm-opt -Oz` on `dist/assets/dioxus/APP_NAME_bg.wasm`. Treat the CLI level as the configure page's claim, and do not invent a header the page does not write.

Output paths disagree across pages (`target/dx/.../bundle`, `dist/bundle`, `public/` plus a `server` binary). After a bundle, trust the path that command printed.

## Bundle

`[bundle]`: `identifier`, `publisher`, `icon`, `resources`, `copyright`, `category`, `short_description`, `long_description`, `external_bin`.

Icons are square PNG, 32-bit RGBA, at 16, 24, 32, 64, and 256. macOS uses `.icns`, Windows `.ico`, Linux `.png`. `external_bin` gets the target triple appended. `category` has to be one of the names in the configure page (Business, DeveloperTool, Education, Entertainment, Finance, Game, and the longer game and lifestyle list on that page).

`[bundle.macos]`: `frameworks`, `minimum_system_version` (default `10.13`), `license`, `exception_domain`, `signing_identity`, `provider_short_name`, `entitlements`, `hardened_runtime`, plus `info_plist_path` and `files` from the deploy config page.

`[bundle.windows]`: `digest_algorithm`, `certificate_thumbprint`, `timestamp_url`, `tsp`, `icon_path` (default `./icons/icon.ico`), `webview_install_mode` (`EmbedBootstrapper`, `DownloadBootstrapper`, `OfflineInstaller` with `silent`, `FixedRuntime` with `path`, or `"Skip"`). The deploy page adds `wix`, `nsis`, `sign_command`, `allow_downgrades`, and `webview_fixed_runtime_path`.

`[bundle.deb]` exists only on the deploy config page: `depends`, `provides`, `conflicts`, `replaces`, `files`, `desktop_template`, `section`, `priority`, scripts, `changelog`.

`dx bundle --desktop --package-types "macos" --package-types "dmg"` is the command the pages actually print. The enum comments also name `"ios"`, `"msi"`, `"nsis"`, `"deb"`, `"rpm"`, `"appimage"`, `"updater"`. The tutorial's claim that only `.exe` can be built for `--desktop` does not match that enum. Do not invent `"exe"` or `"apk"` as `--package-types` values.

JSON output is the last line: `dx bundle --desktop --json-output --verbose | tail -1 | jq -r '.json | fromjson | .BundleOutput.bundles []'`.

## Release size

The optimizing page is written for web. Its TodoMVC sample is 32mb in debug, 2.36mb in release, 310kb with the stable profile, 234kb with the unstable profile. `dx build --release` is the first lever.

`.cargo/config.toml` overrides `Cargo.toml`. Stable `[profile.release]`: `opt-level = "z"`, `debug = false`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `incremental = false`. The unstable profile adds `strip = true`, `build-std` for `std`, `panic_abort`, `core`, and `alloc`, `build-std-features = ["panic_immediate_abort"]`, and rustflags `-Clto`, `-Zvirtual-function-elimination`, `-Zlocation-detail=none`. Those flags drop unwind and location detail. UPX is for native binaries only, not WASM.

Keep dynamic RSX small. Unchanged parts are skipped. `asset!` optimization in release is described as alpha.

## Anti-patterns that show up while wiring an app

Stable list keys. No `Mutex` or `RwLock` in props. No state writes during render. Split a large state struct instead of one store every child reads. Non-deterministic work goes in a hook or an effect, not the component body. `PartialEq` on props returns `false` when the child should update.
