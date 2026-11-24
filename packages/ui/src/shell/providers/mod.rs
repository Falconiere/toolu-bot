//! Providers reference preview; account state is illustrative until integrations exist.
mod catalog;
mod detail;
mod page;

use catalog::{default_name, find, matching, Provider, CONNECTED, OTHER};
use dioxus::prelude::*;
use page::ProviderPage;

/// Searchable provider settings preview shared by every platform.
#[component]
pub(super) fn Providers(mut default_provider: Signal<usize>) -> Element {
  let mut open = use_signal(|| None::<&'static str>);
  if let Some(page) = open().and_then(find) {
    let index = default_provider();
    let chosen = page.index;
    return rsx! {
      ProviderPage {
        key: "{page.provider.name}",
        provider: page.provider,
        can_default: chosen.is_some(),
        is_default: chosen == Some(index),
        default_name: default_name(index).to_string(),
        on_close: move |_| open.set(None),
        on_make_default: move |_| {
            if let Some(index) = chosen {
                default_provider.set(index);
            }
        },
      }
    };
  }
  rsx! {
    ProviderIndex { default_provider, on_open: move |name| open.set(Some(name)) }
  }
}

#[component]
fn ProviderIndex(default_provider: Signal<usize>, on_open: EventHandler<&'static str>) -> Element {
  let mut query = use_signal(String::new);
  let search = query().trim().to_lowercase();
  let connected = matching(&CONNECTED, &search);
  let other = matching(&OTHER, &search);
  let empty = connected.is_empty() && other.is_empty();

  rsx! {
    main { class: "tb-providers", "aria-labelledby": "providers-title",
      div { class: "tb-providers-content",
        h1 { id: "providers-title", "Providers" }
        DefaultProvider { default_provider }
        section { "aria-labelledby": "connected-title",
          div { class: "tb-providers-heading",
            h2 { id: "connected-title", "Connected" }
            label { class: "tb-provider-search",
              span { class: "sr-only", "Search providers" }
              svg { view_box: "0 0 24 24", "aria-hidden": "true",
                circle { cx: "10.5", cy: "10.5", r: "6.5" }
                path { d: "m16 16 4 4" }
              }
              input {
                r#type: "search",
                placeholder: "Search providers",
                value: "{query}",
                oninput: move |event| query.set(event.value()),
              }
            }
          }
          ProviderList { providers: connected, on_open }
        }
        if !other.is_empty() {
          section {
            class: "tb-providers-other",
            "aria-labelledby": "other-providers-title",
            h2 { id: "other-providers-title", "Other providers" }
            ProviderList { providers: other, on_open }
          }
        }
        if empty {
          p { class: "tb-provider-empty", role: "status",
            "No providers match “{query}”. Try another name."
          }
        }
        p { class: "tb-provider-preview",
          "Demo data. Connections aren’t active; the default selection lasts until you close the app."
        }
      }
    }
  }
}

#[component]
fn DefaultProvider(mut default_provider: Signal<usize>) -> Element {
  let selected = CONNECTED[default_provider()];
  rsx! {
    div { class: "tb-provider-default",
      div {
        label { r#for: "default-provider", "Default provider" }
        p { "New agents start with this provider." }
      }
      div { class: "tb-provider-select",
        img { src: selected.logo, alt: "" }
        select {
          id: "default-provider",
          value: "{default_provider}",
          onchange: move |event| {
              if let Ok(index) = event.value().parse::<usize>() {
                  if index < CONNECTED.len() {
                      default_provider.set(index);
                  }
              }
          },
          for (index, provider) in CONNECTED.iter().enumerate() {
            option {
              value: "{index}",
              selected: index == default_provider(),
              "{provider.name}"
            }
          }
        }
        svg { view_box: "0 0 16 16", "aria-hidden": "true",
          path { d: "m5 6 3-3 3 3M5 10l3 3 3-3" }
        }
      }
    }
  }
}

#[component]
fn ProviderList(providers: Vec<Provider>, on_open: EventHandler<&'static str>) -> Element {
  rsx! {
    if !providers.is_empty() {
      div { class: "tb-provider-list",
        for provider in providers {
          button {
            key: "{provider.name}",
            class: "tb-provider-row",
            r#type: "button",
            onclick: move |_| on_open.call(provider.name),
            img {
              class: "tb-provider-logo",
              src: provider.logo,
              alt: "",
            }
            span { class: "tb-provider-copy",
              span { class: "tb-provider-name", "{provider.name}" }
              span { class: "tb-provider-status", "{provider.status}" }
            }
            svg {
              class: "tb-provider-chevron",
              view_box: "0 0 16 16",
              "aria-hidden": "true",
              path { d: "m6 4 4 4-4 4" }
            }
          }
        }
      }
    }
  }
}
