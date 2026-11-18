---
name: dioxus-state
description: >-
  Manage Dioxus 0.7 state with signals, memos, effects, resources, loaders,
  actions, context, and stores. Use whenever adding or changing use_signal,
  use_memo, use_effect, use_resource, use_loader, use_action, use_context,
  use_store, event handlers, suspense, or error boundaries, even if the user
  only says "state", "hook", or "re-render".
---

# Dioxus 0.7 state

Source: [The Basics of State](https://dioxuslabs.com/learn/0.7/essentials/basics/). This repo uses `dioxus` 0.7.1.

Reactivity is a subscription, not a framework-wide store. A component rerenders when a value it read during render is written. Data flows down. Derived values are computed, not stored in a second signal that you write while rendering.

`use_state`, `use_ref`, `cx`, `Scope`, Fermi (`Atom`, `use_read`), and dependency-tuple `use_future` are gone. Their replacements are below. See `../dioxus-migration/SKILL.md` for the full rename list.

## Signals

```rust
let mut count = use_signal(|| 0);
let current = count.read().clone();

rsx! {
    button {
        onclick: move |_| *count.write() += 1,
        "Count: {count}"
    }
}
```

`use_signal` stores the value on the component owner and disposes it on unmount. The handle is `Copy`. `.read()` borrows; `.write()` needs `mut` because it takes `&mut Signal`. Overlapping read and write guards do not compile. Holding either guard across `.await` panics. Drop the guard or clone the inner value first.

These reads subscribe: `.read()`, `"{count}"`, `.iter()`, `count()` when `T: Clone`, and arithmetic (`count += 1` on numeric signals). `peek` reads without subscribing. A signal that no render reads does not rerender that component.

`.set(value)` replaces the value. `with_mut` exists on `Writable` (`count.with_mut(|n| *n += 1)`). The guide's examples use `.write()` and `+=`. Writes are batched until the next step. `.await` is a step boundary. The runtime prefers a paint before polling more futures.

`ReadSignal<T>` is the read-only prop type. `ReadOnlySignal<T>` is a type alias of it. `WriteSignal` is the writable equivalent of `Signal`. Pass `ReadSignal` down, not a concrete `Signal` or `Memo`, so any readable source can be passed. The `rsx!` macro converts `Signal` and `Memo` into `ReadSignal`.

Do not pass `mut signal: Signal<T>` into a child so the child can write. Pass an `EventHandler` and let the owner write. A plain `bool` prop is not reactive: an effect will not rerun when the parent changes it.

`Signal::new()` outside `use_signal` lives until the current owner unmounts unless you `.dispose()` it. `Signal::global(|| 0)` in a `static` is a `GlobalSignal<T>`, one per app instance (one SSR request, one desktop window). Read it as `"{COUNT}"`, write it as `*COUNT.write() += 1`.

## Derived values

| Hook | Use it when | Notes |
|---|---|---|
| `use_memo` | The value is sync and derived | Reruns when reads change. Stores the new value only if `PartialEq` says it changed. Readers subscribe to the memo, not to every input |
| `use_effect` | You must touch the outside world after paint | Document title is the usual case. Prefer a handler when the component stays mounted |
| `use_resource` | Client async work | Restarts when a signal it read is written. Not memoized. The future must be cancel-safe |
| `use_loader` | `Result` you want to `?` into suspense or an error boundary | Returns `Result<Loader<T>, Loading>`. Does not re-suspend when the future reruns. Prefer this over `use_resource` for hybrid client/server |
| `use_action` | The user triggers the future | `.call(args)`, `.value()` is `None`, `Some(Ok(_))`, or `Some(Err(_))`. A new `.call()` cancels the in-flight task |
| `spawn` | Fire-and-forget inside a handler | Returns `Task`. Cancelled on unmount. `spawn_forever` is rooted and is not |

`use_future` is not the 0.7 data hook. Event handlers that return a future are spawned. Futures run on the main thread, may be `!Send`, and must not block. Desktop and mobile use Tokio. Blocking work goes to `std::thread::spawn` on native and a worker on the web.

There is no dependency tuple. The closure subscribes to whatever it reads. Read the dependency in the closure, then move the cloned value into the async block.

## Hooks

`use_hook(|| init)` is the primitive. The initializer runs once; later renders clone the slot. Call hooks at the top level of a component or custom hook, in the same order every time. Not in a condition, loop, or closure. Breaking the order panics.

An early `return` between hooks is allowed, unlike React, because `?` feeds error and suspense boundaries. It is still a bad default. Run every hook, then return.

Custom hooks are `use_*` functions that compose other hooks. `use_drop` and `Runtime`, `queue_effect`, and `provide_root_context` left the prelude in 0.7. Import them explicitly. The guide does not name their new module paths.

## Context

`use_context_provider(|| value)` and `use_context::<T>()` key on `TypeId`. Two `String` contexts collide. Wrap the value in a newtype or one struct of signals:

```rust
#[derive(Clone, Copy)]
struct HeaderContext {
    title: Signal<String>,
    subtitle: Signal<String>,
}
```

The value must be `Clone`. Replacing an existing context entry is not reactive. The guide does not say what `use_context` does when nothing was provided. Do not guess a panic or an `Option`; provide the context above the consumer.

## Stores

Use a store when a write to one field or one map entry should not rerender every consumer. `#[derive(Store)]` plus `use_store(|| HeaderState { ... })`. `header.title()` returns a lens. Lenses decay to `ReadSignal<T>` or `ReadStore<T>` at a component boundary. Nested fields only zoom if the inner type also derives `Store`.

`Option` and `Result` lenses use `.unwrap()`, `.transpose()`, and `.as_ref()`. `.as_ref()` reads and stops further zooming. The only collection API the guide spells out is `Store<HashMap<K, V>>`: `users.iter()` yields entries, insert or remove rerenders once, and editing one entry rerenders that row.

## Errors and suspense

`Element` is `Result<VNode, RenderError>`. `?` and `.context("...")` bubble to the nearest `ErrorBoundary`. `use_resource` needs a `Clone` error, so return `dioxus::Ok(value)` (`Result<T, CapturedError>`).

```rust
ErrorBoundary {
    handle_error: |error: ErrorContext| rsx! { "Failed: {error.error():?}" },
    Child {}
}
```

`SuspenseBoundary` shows its loading closure while a child has suspended. `.suspend()?` on a resource suspends. `use_loader`'s `?` suspends or throws. Start every loader before the first `?` or you get a waterfall.

Read [references/async-and-events.md](references/async-and-events.md) for event payloads, actions, and the loader versus server-future split.
