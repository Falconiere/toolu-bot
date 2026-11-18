# 0.6 renames that still show up in old code

From https://dioxuslabs.com/learn/0.7/migration/to_06/. These are not 0.7 breaks. They are the previous break, and old snippets still use the left side.

## Launch and address

`dioxus::launch` starts the logger when the logging feature is on: `Debug` in development, `Info` in release. Call `dioxus::logger::initialize_default()` only when you need a level before `launch`. The 0.7 utilities page spells the same idea `dioxus::logger::init(Level::INFO)`.

The server binds `IP` and `PORT`. `dioxus_fullstack::Config::addr` is gone. In development the CLI reverse-proxies fullstack. A custom Axum server reads `dioxus_cli_config::fullstack_address_or_localhost()`, or `server_ip` and `server_port`.

## Serve builder

`fullstack::Config` was removed. `LaunchBuilder::new().with_cfg(...).launch(app)` takes `server_only!`, `web!`, and `desktop!`.

Root id moved off that config:

- `ServeConfigBuilder::default().root_id("app")`
- `dioxus::web::Config::default().rootname("app")`
- `dioxus::desktop::Config::default().with_root_name("app")`

`serve_dioxus_application` takes the component, not a vdom factory. `ServeConfigBuilder::build` returns `Result`. A missing `index.html` on desktop may be ignored, and the builder itself can be passed through. `RenderHandleState::new` gained a `ServeConfig` argument. `register_server_fns` is `register_server_functions`.

## Eval

`eval` left the prelude and left the `dioxus-html` `eval` feature. Call `document::eval`. Custom renderers supply `Rc<dyn Document>`. `Eval::recv` and `Eval::join` return any `DeserializeOwned`, not only `serde_json::Value`. `Eval::send` takes any `Serialize`. Liveview cannot `prevent_default` from Rust; that platform needs JavaScript in the handler.

## Globals and signals

`GlobalSignal<T>` is `Global<Signal<T>, T>`. `GlobalMemo<T>` is `Global<Memo<T>, T>`. `.signal()` and `.memo()` on a global became `.resolve()`. `Readable::peek_unchecked` became `try_peek_unchecked`. `Signal::global_memo` requires `T: PartialEq`. The `generational-box` feature `check_generation` was removed.

## Renderer and assets

`dioxus-html` features `web` and `native` were removed. `Template::name` was removed. `into_vcomponent(render_fn, component_name)` lost the name argument. `VNode::template` is a `Template`, not a `Cell<Template>`. `AttributeValue::Any` holds `Rc<dyn AnyValue>`, not `Box`.

`dioxus-hot-reload` became `dioxus_devtools` (`connect`). `panic_hook` moved from `dioxus-web` to `dioxus-logger` and stays on by default.

Asset paths are absolute from the crate root: `asset!("/assets/a.png")`, optional options after the path. The 0.5 form `asset!(image("./assets/a.png").size(100, 100))` is neither the 0.6 nor the 0.7 builder.

## Router traits

`ToRouteSegments` takes `&self`. Implement it for the owned type. Custom renderers that implement the file and resize hooks need `FileEngine::file_size` and `HtmlEventConverter::convert_resize_data`. `dioxus_autofmt::write_block_out` takes `&CallBody`.

## What not to "fix"

The 0.6 page writes `Mutations::sanitize` and `dioxus_html::AttributeDescription` on both sides of the arrow. Those names did not change. Leave them.
