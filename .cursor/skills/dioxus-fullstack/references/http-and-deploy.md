# Middleware, streams, forms, streaming, SSG, errors

From https://dioxuslabs.com/learn/0.7/essentials/fullstack/. `loading.md` and `routes.md` are empty. `headers.md` is an unfinished note that names `SetHeader<T>`, `Redirect`, and `FullstackContext::response_headers` and then says client headers are "currently not possible." Do not treat that page as a specified API.

## Middleware

Axum layers go on the router inside `dioxus::serve`. `#[middleware]` on one endpoint builds a `MethodRouter` and calls `.layer()`. The guide shows `axum::middleware::from_fn(|request, next| async move { next.run(request).await })` and a Tower `ServiceBuilder` (sensitive headers, trace, extensions, compression, header propagation, bearer and accept validation). Put a session layer inside a cache layer only if you mean to cache personalized HTML. Caching is URL-based. Auth lives in headers.

## Websockets

```rust
#[get("/api/upper")]
async fn uppercase_ws(options: WebSocketOptions) -> Result<Websocket> { /* ... */ }
```

`Websocket<In = String, Out = String, E = JsonEncoding>`. `options.on_upgrade` runs on a tokio `LocalSet`, so the future need not be `Send`. `.send()` / `.recv()` go through the encoding. `send_raw` sends frames. Encodings: `JsonEncoding` (default), `CborEncoding`, plus `PostcardEncoding` and `MsgPackEncoding` on streams. The same type is spelled `WebsocketOptions` in the extractor chapter.

Client: `use_websocket(|| uppercase_ws(WebSocketOptions::new()))`. The handle has `.status()`, `.send()`, `.recv()`, and `.set()` with a new socket to restart. Browsers generally cannot stream a request body and a response body together. Use a stream one way, a websocket both ways.

## Streams and files

`TextStream::spawn(|tx| async move { tx.unbounded_send(...) })` or `Streaming::new(rx)`. `ByteStream` is consumed with `.next().await`. A server function can return `Result<Streaming<Dog, JsonEncoding>>` via `Streaming::spawn`. `FileStream` is not that type. `FileStream::from_path(...).await?` sets `Content-Disposition` and `X-Content-Size`. Upload with `file.into()` into `FileStream`.

## Forms

`Form<T>` where `T: Serialize + Deserialize`. On submit, `prevent_default()`, then `evt.parsed_values()` into the struct (fields need `name`), then `login(Form(values)).await`. `GET` puts values in the URL. Use `POST` for anything nested. Multipart: `evt.into()` into `MultipartFormData`, then `form.next_field().await`, `field.name()`, `field.file_name()`, `field.content_type()`, `field.bytes().await`. A typed multipart struct is not supported.

## Auth

No built-in auth. The documented pattern is middleware that reads `Authorization`, loads a session, and `request.extensions_mut().insert(session)`. The named stack is `axum_session_auth` with `sqlx`: `AuthLayer` and `SessionLayer`. Extractor methods shown: `auth.login_user(2)`, `auth.logout_user()`.

## HTML streaming

Off by default. One HTML document is sent after every suspense boundary resolves. `ServeConfig::enable_out_of_order_streaming` turns streaming on, passed through `LaunchBuilder::with_cfg` or `serve_dioxus_application`. Users without JavaScript do not see streamed chunks. Turn it off for that audience.

`document::Link`, `document::Meta`, and `document::Title` must be in the first chunk, the one that contains `<head>`. Later head elements render on the client and are invisible to crawlers. The first chunk is sent when `commit_initial_chunk` runs. With the router, that is when every suspense boundary above the router has resolved. After that call, status and headers are frozen. A `404` from `use_loader` / `HttpError` becomes the response status only if it surfaces before the commit. Inside a `SuspenseBoundary`, it does not.

## Error status

An error that reaches the root is downcast to a status. An `ErrorBoundary` swallows that unless you commit it:

```rust
ErrorBoundary {
    handle_error: move |err: ErrorContext| {
        let http_error = FullstackContext::commit_error_status(err.error().unwrap());
        match http_error.status {
            StatusCode::NOT_FOUND => rsx! { div { "404" } },
            _ => rsx! { div { "Error" } },
        }
    },
    Outlet::<Route> {}
}
```

Put that boundary in a `#[layout]` so it wraps every page.

## SSG

Enable incremental rendering on `ServeConfig` and render into `public` (next to the server binary). The CLI calls a server function whose endpoint is the string `"static_routes"` and expects `Vec<String>`. `Routable::static_routes()` lists routes with no dynamic segments. Bundle with `dx bundle --web --ssg`. The guide says a `Cache-Control` header plus a CDN is enough for many sites, and that SSG does not help an app that is mostly a client tool.

## Deploy

`dx bundle --web --release` writes `public/` plus a `server` binary. The server reads `IP` and `PORT` (Docker: `IP=0.0.0.0`, `PORT=8080`). The HotDog tutorial deploys that binary to Fly.io and mounts the sqlite file. WASM server hosts (Cloudflare Workers, Fermyon Spin) need a manual WASM server build. The old `publishing.md` recipe (`dx build --release --features web` then `cargo run --features ssr`) uses the feature name `ssr`. The current feature name is `server`.
