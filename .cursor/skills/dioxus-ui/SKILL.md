---
name: dioxus-ui
description: >-
  Write Dioxus 0.7 RSX, components, styling, and assets. Use whenever editing a
  Dioxus component, rsx!, #[component], CSS, Tailwind, asset!, document::Stylesheet,
  lists, conditionals, or HTML-in-Rust UI, even if the user never says "RSX".
  Dioxus 0.7 only: no cx, Scope, use_state, or render!.
---

# Dioxus 0.7 UI

Source: [Building User Interfaces](https://dioxuslabs.com/learn/0.7/essentials/ui/). This repo depends on `dioxus` 0.7.1.

A component is a pure function from owned props to an `Element` (`Result<VNode, RenderError>`). `rsx!` builds a virtual tree. The renderer diffs it. Static nodes are skipped; only dynamic parts are checked. That is why keys, `PartialEq`, and "no writes during render" decide whether the UI stays correct.

`cx`, `Scope`, `use_state`, and `render!` are gone. Return `rsx!` from the function. Open `../dioxus-migration/SKILL.md` if the surrounding code still uses them.

## Components

Annotate with `#[component]`. The name starts with a capital letter or contains an underscore. Do not call it as a normal function; RSX creates the instance that owns its hooks.

Props are function arguments, or one `props: CardProps` argument with `#[derive(PartialEq, Clone, Props)]`. They are cloned every render and compared with `PartialEq` to decide whether the child rerenders. Owned values only: `String`, not `&str`. A plain helper that returns `Element` and is not a component may still take `&str`.

| Prop shape | What the macro does |
|---|---|
| `Option<T>` | Optional, defaults to `None` |
| `#[props(default)]` | Optional, uses `Default` |
| `#[props(!optional)]` | `Option<T>` is required |
| `#[props(into)]` | Converts with `Into` |
| `ReadSignal<T>` / `ReadOnlySignal<T>` | Accepts a signal, a memo, or a plain `T`. `ReadOnlySignal` is an alias of `ReadSignal` |
| `String` | Accepts formatted strings. `#[props(into)]` on `String` is redundant |
| `children: Element` | Nested RSX. Render it as `{children}` |
| `EventHandler<T>` | Accepts a closure. Invoke with `.call(value)` |
| `#[props(extends = GlobalAttributes)]` | Field type `Vec<Attribute>`. Spread with `..props` |

Doc comments on arguments become hover docs. Manual `PartialEq` is allowed when only one field should matter. If you are unsure whether a change should rerender the child, return `false` from `PartialEq`. Returning `true` after a real change leaves stale UI.

Spread component props as `Card { title: "Chapter 1", ..props }`. Later fields win. Builder outside RSX: `CardProps::builder().content("body".to_string()).build()`.

## RSX

Elements look like Rust struct literals: `div { class: "container", "Hello" }`. Text uses `format!` rules and may embed any Rust expression: `"Hello, {name}"`.

Attributes are `name: value`. `class` may be repeated and each one is added to the class list. Every other attribute appears once. A conditional attribute is an unterminated `if`:

```rust
div {
    class: "btn",
    class: if active() { "btn-active" },
    width: if wide() { "100%" },
    "Save"
}
```

`if` and `for` inside `rsx!` are RSX, not Rust. Their bodies are elements. An `if` without `else` still renders (an empty placeholder). Prefer that over wrapping an iterator in braces. An iterator must be wrapped: `{(0..5).map(|i| rsx! { span { "{i}" } })}`.

`Signal` collections need `.iter()` before `for`. Every list item needs a stable `key` tied to the item, not the index. A missing key is only acceptable when the list never changes. If a loop has several roots, the first element's key is the identity; otherwise wrap them in a `Fragment` (`children` returned as-is).

```rust
for user in users.iter() {
    User { key: "{user.id}", id: user.id }
}
```

Expressions in `{...}` must be `Element` or `IntoDynNode` (`Component`, `Text`, placeholder `None`, `List`, `Option<T>`). You may `return Err("missing".into())` while building an `Element`.

Events are `on*` closures: `onclick: move |_| count += 1`. `onresize` and `onvisible` are Dioxus events. Quote custom attribute names: `"data-count": 1`. A quoted name can be a listener, including a JavaScript string. Spread element attributes with `..attributes`.

`false` removes some boolean attributes. To force an HTML boolean attribute on, use `Some("scope")` as in `itemscope: Some("scope")`.

In 0.7, form submit is allowed by default. Stop a reload with `onsubmit: move |e| e.prevent_default()`. Desktop still blocks page navigation on its own.

## Rendering

Rerender when props fail `PartialEq`, or when a signal this component read during render is written. A signal that is only read inside a handler does not rerender this component. Do not write signals, context, or hooks while rendering; an unconditional write rerenders forever. Put mutations in handlers or in `use_effect` (after paint).

Do not pass a child a reference into a signal. Clone the value, pass the collection plus an index, or pass a `ReadSignal`.

## Styles and assets

Read [references/assets-and-style.md](references/assets-and-style.md) before changing CSS, Tailwind, images, hot reload, or raw HTML. The short form:

```rust
static MAIN_CSS: Asset = asset!("/assets/main.css");

rsx! {
    document::Stylesheet { href: MAIN_CSS }
    img { src: asset!("/assets/static/ferrous_wave.png"), alt: "Wave" }
}
```

`asset!` paths start with `/` and are relative to the crate root. A machine path is rejected. An unused `Asset` is dropped by the compiler unless something references it or it is `#[used]`.

If the project root contains `tailwind.css`, `dx` starts Tailwind itself and writes `assets/tailwind.css`. Load that file with `document::Stylesheet`. Dynamic class names hidden inside a formatted string are invisible to the Tailwind compiler; put them in their own `class` attribute.
