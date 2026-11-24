//! Providers reference preview; account state is illustrative until integrations exist.
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
struct Provider {
  name: &'static str,
  status: &'static str,
  logo: Asset,
}

const CONNECTED: [Provider; 8] = [
  Provider {
    name: "ChatGPT",
    status: "you@example.com",
    logo: asset!("/assets/providers/chatgpt.svg"),
  },
  Provider {
    name: "Claude",
    status: "you@example.com",
    logo: asset!("/assets/providers/claude.svg"),
  },
  Provider {
    name: "Grok",
    status: "you@example.com",
    logo: asset!("/assets/providers/grok.svg"),
  },
  Provider {
    name: "OpenCode",
    status: "Runs on Agents - VPS",
    logo: asset!("/assets/providers/opencode.svg"),
  },
  Provider {
    name: "Gemini",
    status: "you@example.com",
    logo: asset!("/assets/providers/gemini.svg"),
  },
  Provider {
    name: "Cursor",
    status: "Runs on Agents - VPS",
    logo: asset!("/assets/providers/cursor.svg"),
  },
  Provider {
    name: "Muse",
    status: "you@example.com",
    logo: asset!("/assets/providers/terminal.svg"),
  },
  Provider {
    name: "OpenRouter",
    status: "Runs on Agents - VPS",
    logo: asset!("/assets/providers/openrouter.svg"),
  },
];

const OTHER: [Provider; 2] = [
  Provider {
    name: "Cline",
    status: "Not downloaded",
    logo: asset!("/assets/providers/cline.svg"),
  },
  Provider {
    name: "Pi",
    status: "Not connected",
    logo: asset!("/assets/providers/terminal.svg"),
  },
];

/// Searchable provider settings preview shared by every platform.
#[component]
pub(super) fn Providers(default_provider: Signal<usize>) -> Element {
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
          ProviderList { providers: connected }
        }
        if !other.is_empty() {
          section {
            class: "tb-providers-other",
            "aria-labelledby": "other-providers-title",
            h2 { id: "other-providers-title", "Other providers" }
            ProviderList { providers: other }
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
fn ProviderList(providers: Vec<Provider>) -> Element {
  rsx! {
    if !providers.is_empty() {
      div { class: "tb-provider-list",
        for provider in providers {
          details { key: "{provider.name}", class: "tb-provider-row",
            summary {
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
            p { class: "tb-provider-detail",
              "{provider.name} is shown as a preview. Provider sign-in and installation are not available yet."
            }
          }
        }
      }
    }
  }
}

fn matching(providers: &[Provider], query: &str) -> Vec<Provider> {
  providers
    .iter()
    .copied()
    .filter(|provider| {
      provider.name.to_lowercase().contains(query) || provider.status.to_lowercase().contains(query)
    })
    .collect()
}
