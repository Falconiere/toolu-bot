---
name: dioxus-router
description: >-
  Add Dioxus 0.7 routing with a Routable enum, Link, layouts, and nested
  routes. Use whenever the user mentions routes, pages, URLs, navigation,
  Link, Outlet, or layouts in a Dioxus app, even if they do not say "router".
---

# Dioxus 0.7 router

Source: [Routing](https://dioxuslabs.com/learn/0.7/essentials/router/). Enable it with `dioxus` feature `router`. This repo's fullstack crates also enable `fullstack`.

The route enum is the URL parser, the URL formatter, and the component map. A typo in a path fails at compile time instead of as a blank page. An `<a href>` does none of that: it full-reloads, it does not work on desktop, and it cannot prove the target exists.

```rust
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},

    #[route("/blog/:id")]
    BlogPost { id: i32 },
}

#[component]
fn NavBar() -> Element {
    rsx! {
        Link { to: Route::Home {}, "Home" }
        Outlet::<Route> {}
    }
}

fn main() {
    dioxus::launch(|| rsx! { Router::<Route> {} });
}
```

`Router` takes the enum as its generic. `Outlet::<Route>` is where the matched child renders. Without an outlet, the match renders where `Router` is. `#[layout(NavBar)]` renders that component first and applies to the following variants, which is why the outlet has to live in the layout.

## Matching a component

By default the variant renders the in-scope component of the same name. Pass a different component as the second argument:

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/", HomePage)]
    Index {},
}
```

A layout must take a prop for each dynamic segment on the routes it wraps, or read the route with `use_route`.

## Segments

Declare specific routes before general ones. Static, then dynamic, then catch-all. If two routes match, enum order wins. Query and hash are always matched.

| Segment | Syntax | Parse |
|---|---|---|
| Static | `"/about"` | Fixed string |
| Dynamic | `":id"` | `FromStr`. A failed parse tries the next route |
| Catch-all | `":..segments"` | `Vec<String>` via `FromSegments`. Last path segment. Cannot sit inside `#[nest]` |
| Query | `"/search?:q&:page"` | `FromStr + Default`. A missing or bad value becomes `Default`, and the route still matches |
| Query spread | `?:..query` | The whole query string |
| Hash | `"#:fragment"` | `FromStr + Default`. Same failure rule as query |

There is no `:id?` optional path parameter. Use an enum variant per shape, or `Option` on a query.

`#[nest("/settings")]` prefixes every child until `#[end_nest]`, or until the enum ends. The nest path cannot contain a catch-all or a query. A dynamic segment on the nest is in scope for every child and layout. That is how `/settings`, `/settings/password`, and `/settings/privacy` share one prefix and one layout.

`#[redirect]` takes a path and a closure over every parameter in that path. The closure returns a `NavigationTarget`. The guide's example maps `/myblog` and `/myblog/:id` onto `/blog` and `/blog/:id`. The closure body lives in a docs include that is not in the markdown, so copy a redirect from the live router page instead of inventing the closure.

An unmatched URL renders nothing. Add a catch-all variant last (`PageNotFound` with a `segments` parameter in the tutorial) if a blank page is not acceptable.

## Navigation

`Link { to: Route::About {}, "About" }` is the anchor. `to` is `NavigationTarget`. The live router page and `dioxus-router` 0.7 both use `to`. A later chapter says `target`. Write `to`.

`navigator()` returns `Navigator`. `push` adds a history entry. `replace` overwrites the current one, so Back will not return. Both take a `NavigationTarget` (`Internal` from a `Routable` variant, or `External` from a URL). External navigation can fail when the URL is invalid. `GoBackButton` and `GoForwardButton` are the history controls. There is no `GoBack` component.

`RouterConfig`'s `on_update` runs after the router updates its state and before dependent components. If it returns a `NavigationTarget`, the router replaces the location and does not call `on_update` again. A failed navigation (bad target, bad redirect, bad `on_update` target) goes to the failure state and does not call `on_update`.

History defaults to `MemoryHistory`. `WebHistory` is the browser URL. `LiveviewHistory` is liveview. Switching to `WebHistory` just because the `web` feature is on is not guaranteed. `WebHistory` cannot disable the back button, because the browser History API does not report emptiness, and the user can leave the app.

With the `web` feature, every field of a `Routable` variant has to appear in its `#[route(...)]`.

## Fullstack

A layout that owns the `ErrorBoundary` should call `FullstackContext::commit_error_status` or the HTTP status is swallowed. Streaming freezes head tags and status at `commit_initial_chunk`, which the router fires when the suspense boundaries above it resolve. See the fullstack skill.
