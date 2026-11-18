---
name: dioxus-fullstack
description: >-
  Build Dioxus 0.7 fullstack apps: server functions, SSR, hydration, Axum,
  streaming, SSG, websockets, forms, and native clients. Use whenever the user
  mentions #[server], #[get], #[post], use_server_future, use_loader, SSR,
  hydration, Axum, server functions, or a Dioxus backend, even if they only
  say "API" or "fullstack".
---

# Dioxus 0.7 fullstack

Source: [Fullstack](https://dioxuslabs.com/learn/0.7/essentials/fullstack/). This repo is a workspace: shared server functions live in `packages/api`, shared UI in `packages/ui`, and `packages/web`, `packages/desktop`, and `packages/mobile` are the platform crates. `dioxus` is 0.7.1.

A fullstack app is two binaries. The `server` feature enables `dioxus/server`. A client feature (`web`, `desktop`, `mobile`) enables the matching renderer. `fullstack` on the `dioxus` dependency tells `dx` to build both. The feature is not spelled `dioxus/fullstack`.

```toml
[dependencies]
dioxus = { version = "0.7.1", features = ["fullstack"] }

[features]
default = []
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
server = ["dioxus/server"]
```

After this split, serve a platform on purpose: `dx serve --web`, `dx serve --desktop`. Server-only crates (`tokio`, `rusqlite`, `sqlx`) are `optional = true` and listed only on the `server` feature (`dep:tokio`). Otherwise the WASM client fails to link. Client-only crates (`web-sys`) hang off `web`. A shared server-function crate is imported with `default-features = false`.

Code behind `#[cfg(feature = "server")]` is absent from the client bundle. Code that is not behind that cfg is in the client bundle. Do not call `web-sys` inside a server function or before `launch`. Changing features is not hot-reloaded; restart `dx`.

## Server functions

The function is `async` and returns `Result<T, E>`. Arguments and the ok type are `Serialize + Deserialize`, or the request/response conversion traits. Prelude `Result<T>` is anyhow's `Result`.

Use an explicit method and path. `#[server]` hashes the path (`/api/do_it12nldkj…`). That hash changes when the signature or body changes, which breaks desktop and mobile clients. `deploy.md` says a `#[server]` function can pin `endpoint = "xyz"`. Prefer `#[get]` / `#[post]` / `#[put]` / `#[delete]` / `#[patch]`.

```rust
#[get("/api/products/{product}?color&quantity")]
async fn get_product_data(
    product: String,
    color: String,
    quantity: Option<i32>,
) -> Result<Vec<Product>> {
    // ...
}
```

Path and query follow Axum syntax. `Option<T>` is a valid query. Stick to simple types. Several serializable arguments become one JSON body. The default codec is JSON. The 0.6 default was URL-encoded form data, which cannot send empty or nested structures. Bare `#[server]` is JSON. The upgrade guide's prose says `protocol = Http<PostUrl, Json>` to keep the old form encoding, and its sample writes `Http<GetUrl, Json>`. Those two strings disagree. Do not pick one unless the caller still needs form encoding.

`ServerFnError` is the Dioxus type (it converts into `dioxus::Error`), not the `server_fn` crate's type. Match `ServerFnError::ServerError { code, .. }`. `StatusCode`, `HttpError`, and `CapturedError` also downcast out of anyhow. Unknown errors become HTTP 500. Helpers on `Result`, `Option`, and `bool`: `.or_unauthorized("...")?` and `.or_http_error(StatusCode::UNAUTHORIZED, "...")?`.

Server-only extractors are extra arguments on the attribute, not Rust parameters the client passes. They implement `FromRequestParts` (or `FromRequest` when there is no client body):

```rust
#[get("/api/users/{user_id}", db: SqlDb)]
async fn load_user(user_id: i32) -> Result<User> { /* ... */ }
```

Adding a required argument on a path that a shipped native app already calls will break that app. Make the new field `Option<T>`, or publish a new path (`/api/v2/...`).

## Calling them

| Call site | When |
|---|---|
| `endpoint().await` inside an async handler | One-shot from a click |
| `use_action(endpoint)` then `.call(args)` | The user triggers it. See the state skill |
| `use_loader(move \|\| endpoint(id()))?` | Hybrid load. `?` suspends or throws |
| `use_server_future` | The value is rendered during SSR. The server result is serialized into the HTML |
| `use_server_cached` | The value is non-deterministic and must be stable across hydration |

`use_server_future` and `use_server_cached` always run on the server. The client may skip the closure. Do not put side effects in them. `use_resource(...).suspend()?` is not the SSR path: the wait is non-deterministic and the HTML will not match.

## Hydration

The first client render must match the server HTML. A mismatch shows up as `Cannot set properties of undefined (setting 'textContent')` or a deserialize error that names the server type and the client type. Non-deterministic reads go in `use_server_future`, `use_server_cached`, or `use_effect`. Browser APIs go in `use_effect`, which runs after hydration.

## Axum

`dioxus::launch` is the default server. `dioxus::serve` is the custom one. Gate it:

```rust
fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        Ok(dioxus::server::router(app))
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(app);
}
```

`dioxus::server::router` installs assets, SSR, and server functions. SSR is the fallback, so routes you add win. Import `dioxus::server::axum::routing::{get, post}`. The extension form is `.register_server_functions().serve_static_assets().fallback(...)`.

`Lazy<T>` in `dioxus::fullstack` is the documented way to open a pool without a custom `serve` (`Lazy<sqlx::SqlitePool>`, initialized with `Lazy::new(|| async move { dioxus::Ok(...) })`). `LazyLock` is the sync alternative.

The HotDog tutorial uses `rusqlite` 0.32, one `Connection` per thread in a `thread_local`, database file `hotdog.db`. The guide's recommendation for most apps is PostgreSQL or MySQL through `sqlx`. Diesel is called out as a poor async fit.

## Native clients

Desktop and mobile have no page origin. Set the host before any request, and only off the server binary:

```rust
#[cfg(not(feature = "server"))]
dioxus::fullstack::set_server_url("https://hot-dog.fly.dev");
```

`deploy.md` also prints `server_fn::client::set_server_url`. The essentials page prints `dioxus::fullstack::set_server_url`. Prefer the essentials path.

Native builds do not SSR, hydrate, stream HTML, or SSG. There is no `FullstackContext` during component render. `#[cfg(feature = "server")]` inside the client render does not run.

Read [references/http-and-deploy.md](references/http-and-deploy.md) for middleware, websockets, streams, forms, auth, streaming, SSG, and error status.
