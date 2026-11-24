//! Inner page for one provider: accounts, runtime, models, and use.

use super::catalog::{
  accounts_line, add_account, agent_count, agents_line, owned_accounts, remove_account,
  remove_unused, used_by_line, AccountRow, Provider, ACCOUNT_LIMIT,
};
use super::detail::{Models, UseBar};
use dioxus::prelude::*;

/// One provider's accounts, runtime, models, and whether new agents use it.
#[component]
pub(super) fn ProviderPage(
  provider: Provider,
  can_default: bool,
  is_default: bool,
  default_name: String,
  on_close: EventHandler,
  on_make_default: EventHandler,
) -> Element {
  let mut rows = use_signal(|| owned_accounts(provider));
  let agents = agent_count(&rows.read());
  let hosted = provider.hosted;
  rsx! {
    main { class: "tb-providers", "aria-labelledby": "provider-page-title",
      div { class: "tb-providers-content tb-provider-page",
        Crumb { name: provider.name, on_close }
        Identity { provider, agents }
        Accounts {
          rows,
          on_add: move |_| add_account(&mut rows.write(), hosted),
          on_remove: move |index| remove_account(&mut rows.write(), index),
          on_remove_unused: move |_| remove_unused(&mut rows.write()),
        }
        Runtime {
          version: version_text(provider),
          installed: provider.version.is_some(),
        }
        Models { provider }
        UseBar {
          name: provider.name,
          default_name,
          is_default,
          can_default,
          initially_on: provider.connected,
          on_make_default,
        }
      }
    }
  }
}

fn version_text(provider: Provider) -> String {
  provider.version.unwrap_or("Not installed").to_string()
}

#[component]
fn Crumb(name: String, on_close: EventHandler) -> Element {
  rsx! {
    div { class: "tb-crumb",
      nav { "aria-label": "Breadcrumb",
        button {
          class: "tb-crumb-back",
          r#type: "button",
          onclick: move |_| on_close.call(()),
          "Providers"
        }
        {chevron()}
        span { class: "tb-crumb-current", "aria-current": "page", "{name}" }
      }
      button {
        class: "tb-crumb-close",
        r#type: "button",
        "aria-label": "Close provider",
        onclick: move |_| on_close.call(()),
        {cross()}
      }
    }
  }
}

#[component]
fn Identity(provider: Provider, agents: usize) -> Element {
  rsx! {
    div { class: "tb-provider-identity",
      div { class: "tb-provider-mark",
        img { src: provider.logo, alt: "" }
      }
      div {
        div { class: "tb-provider-title",
          h1 { id: "provider-page-title", "{provider.name}" }
          span { class: badge_class(provider), "{badge_label(provider)}" }
        }
        p { class: "tb-muted", "{agents_line(agents)}" }
      }
    }
  }
}

fn badge_label(provider: Provider) -> &'static str {
  if provider.connected {
    "Connected"
  } else {
    provider.status
  }
}

fn badge_class(provider: Provider) -> &'static str {
  if provider.connected {
    "tb-conn"
  } else {
    "tb-conn tb-conn--off"
  }
}

#[component]
fn Accounts(
  rows: ReadSignal<Vec<AccountRow>>,
  on_add: EventHandler,
  on_remove: EventHandler<usize>,
  on_remove_unused: EventHandler,
) -> Element {
  let count = rows.read().len();
  let unused = rows.read().iter().any(|row| row.used_by.is_empty());
  rsx! {
    section {
      class: "tb-provider-block",
      "aria-labelledby": "provider-accounts",
      div { class: "tb-block-head",
        h2 { id: "provider-accounts", "Accounts" }
        div { class: "tb-provider-actions",
          AddAccount { disabled: count >= ACCOUNT_LIMIT, onclick: on_add }
          Menu {
            label: "Account list actions",
            action: "Remove unused accounts",
            disabled: !unused,
            onclick: on_remove_unused,
          }
        }
      }
      p { class: "tb-block-note", "{accounts_line(count)}" }
      for (index, row) in rows.read().iter().enumerate() {
        AccountCard {
          key: "{row.label}",
          index,
          label: row.label.clone(),
          used_by: row.used_by.clone(),
          on_remove,
        }
      }
    }
  }
}

#[component]
fn AddAccount(disabled: bool, onclick: EventHandler) -> Element {
  rsx! {
    button {
      class: "tb-provider-btn",
      r#type: "button",
      disabled,
      title: if disabled { "20 accounts is the limit." },
      onclick: move |_| onclick.call(()),
      {user_plus()}
      "Add account"
    }
  }
}

#[component]
fn AccountCard(
  index: usize,
  label: String,
  used_by: Vec<String>,
  on_remove: EventHandler<usize>,
) -> Element {
  let line = used_by_line(&used_by);
  rsx! {
    article { class: "tb-provider-card tb-account",
      div { class: "tb-account-top",
        span { class: "tb-account-label", "{label}" }
        Menu {
          label: "Actions for {label}",
          action: "Remove account",
          onclick: move |_| on_remove.call(index),
        }
      }
      if let Some(line) = line {
        p { class: "tb-used-by", "{line}" }
      }
    }
  }
}

#[component]
fn Menu(
  label: String,
  action: String,
  #[props(default)] disabled: bool,
  onclick: EventHandler,
) -> Element {
  rsx! {
    details { class: "tb-menu",
      summary { class: "tb-icon-btn", "aria-label": "{label}", {dots()} }
      div { class: "tb-menu-pop",
        button {
          class: "tb-menu-item",
          r#type: "button",
          disabled,
          onclick: move |_| onclick.call(()),
          "{action}"
        }
      }
    }
  }
}

#[component]
fn Runtime(version: String, installed: bool) -> Element {
  let mut notice = use_signal(String::new);
  rsx! {
    section { class: "tb-provider-block", "aria-labelledby": "provider-runtime",
      h2 { id: "provider-runtime", "Runtime" }
      div { class: "tb-provider-card",
        div { class: "tb-card-row",
          div {
            div { "Version" }
            p { class: "tb-muted", "{version}" }
            if !notice.read().is_empty() {
              p { class: "tb-muted", role: "status", "{notice}" }
            }
          }
          button {
            class: "tb-provider-btn",
            r#type: "button",
            onclick: move |_| notice.set(update_message(installed)),
            "Check for updates"
          }
        }
      }
    }
  }
}

fn update_message(installed: bool) -> String {
  if installed {
    "You're on the latest version.".to_string()
  } else {
    "Nothing installed to update.".to_string()
  }
}

fn user_plus() -> Element {
  rsx! {
    svg { class: "tb-glyph", view_box: "0 0 16 16", "aria-hidden": "true",
      circle { cx: "6", cy: "5", r: "2.1" }
      path { d: "M1.8 13.2a4.4 4.4 0 0 1 8.1 0" }
      path { d: "M12.2 6.8v4.4M10 9h4.4" }
    }
  }
}

fn dots() -> Element {
  rsx! {
    svg { class: "tb-dots", view_box: "0 0 16 16", "aria-hidden": "true",
      circle {
        cx: "3.5",
        cy: "8",
        r: "1.15",
        fill: "currentColor",
      }
      circle {
        cx: "8",
        cy: "8",
        r: "1.15",
        fill: "currentColor",
      }
      circle {
        cx: "12.5",
        cy: "8",
        r: "1.15",
        fill: "currentColor",
      }
    }
  }
}

fn cross() -> Element {
  rsx! {
    svg { class: "tb-glyph", view_box: "0 0 16 16", "aria-hidden": "true",
      path { d: "M4 4l8 8M12 4 4 12" }
    }
  }
}

fn chevron() -> Element {
  rsx! {
    svg {
      class: "tb-glyph tb-crumb-sep",
      view_box: "0 0 16 16",
      "aria-hidden": "true",
      path { d: "m6 4 4 4-4 4" }
    }
  }
}
