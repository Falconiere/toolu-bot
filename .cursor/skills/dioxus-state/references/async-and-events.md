# Events, actions, and async boundaries

From https://dioxuslabs.com/learn/0.7/essentials/basics/ (event_handlers, async, resources, suspense, error_handling).

## Events

Listeners are `on*` closures. The argument is `Event`. Documented listeners include `onclick`, `onmouseover`, `onmousedown`, `onmousemove`, `onkeydown`, `onkeyup`, `onkeypress`, `onsubmit`, `oninput`, `onchange`, `onfocus`, `onblur`, `ondrag`, `ondrop`, `ondragover`, `onscroll`, `onload`, `onresize`.

Accessors the guide names: `.value()`, `.values()`, `.checked()`, `.files()`, `.key()` (`Key`). `evt.prevent_default()` cancels the platform default and still runs handlers. `stop_propagation()` stops the bubble. The default phase is bubbling, inner target first. Web downcast: `evt.downcast::<web_sys::Event>()`.

```rust
input {
    value: "{text}",
    oninput: move |e| text.set(e.value()),
}
form {
    onsubmit: move |e| {
        e.prevent_default();
        values.set(e.values());
    },
}
```

Controlled inputs set `value` from your signal. Uncontrolled inputs keep DOM state. File inputs expose `files` on `FormData`; reading bytes is async. `directory: true` selects a folder.

`EventHandler<T>` carries the runtime, so it can be called off the render thread. `Callback<I, O>` is the variant that returns a value. Async `onclick` futures are spawned and are not cancelled when the user clicks again.

## use_action

`use_action` takes a callback that returns `Result<T>` (the examples use anyhow). `.value()` is matched as:

```rust
match action.value() {
    Some(Ok(res)) => rsx! { "{res:?}" },
    Some(Err(_e)) => rsx! { "Failed" },
    None => rsx! { "Idle" },
}
```

`.call(args)` starts it. A second call cancels the current `Task`.

## use_loader and suspense

`use_loader` returns `Result<Loader<T>, Loading>`. `?` while it is pending suspends the nearest suspense boundary; a `Result` error throws to the nearest error boundary. It does not re-suspend when the future runs again. It does re-throw loading errors.

```rust
let breeds = use_loader(move || async move {
    reqwest::get("https://dog.ceo/api/breeds/list/all")
        .await?
        .json::<ListBreeds>()
        .await
})?;
```

`SuspenseBoundary` takes a loading closure and children. `with_loading_placeholder` supplies a different placeholder. Call every `use_loader` before any `?`.

`use_resource(...).suspend()?` is the client-side suspend. Its wait is not deterministic. Do not use it as the SSR path. Fullstack rendering of server data uses `use_server_future` (see the fullstack skill). That hook suspends by itself. Do not also call `.suspend()`.

## Errors

```rust
pub enum RenderError {
    Error(CapturedError),
    Suspended(SuspendedFuture),
}
```

`CapturedError` wraps `Arc<anyhow::Error>`. `anyhow::Error`, `StatusCode`, `HttpError`, and `ServerFnError` can be downcast. `error_handling.md` shows `.downcast_ref`, `.context()`, `anyhow!()`, and `bail!()`. Local field validation belongs in a memo, not in a thrown error.

## What the guide prefers

Write state in the handler that knows the component is still mounted. Use an effect only when the write must happen because something was read, or must be cleaned up on unmount. Do not copy derived data into a second signal during render. Do not put a `Mutex` or `RwLock` in props: writing the inner value does not rerender anyone. Pass a `Signal` or immutable data.
