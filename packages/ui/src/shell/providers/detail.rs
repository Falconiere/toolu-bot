//! Model picker and the "use this provider" block on a provider page.

use super::catalog::{matching_model_indexes, picker_line, Provider};
use dioxus::prelude::*;

/// Models that show in the picker, with search and hide-all.
#[component]
pub(super) fn Models(provider: Provider) -> Element {
  let mut query = use_signal(String::new);
  let shown = use_signal(|| vec![provider.connected; provider.models.len()]);
  let text = query();
  let visible = matching_model_indexes(provider.models, &text);
  let flags = shown.read().clone();
  let on = flags.iter().filter(|flag| **flag).count();
  rsx! {
    section { class: "tb-provider-block", "aria-labelledby": "provider-models",
      div { class: "tb-block-head",
        h2 { id: "provider-models", "Models" }
        button {
          class: "tb-text-btn",
          r#type: "button",
          onclick: move |_| set_shown(shown, on == 0),
          if on == 0 {
            "Show all"
          } else {
            "Hide all"
          }
        }
      }
      p { class: "tb-block-note", "{picker_line(on, provider.models.len())}" }
      input {
        class: "tb-model-search",
        r#type: "search",
        placeholder: "Search models",
        "aria-label": "Search models",
        value: "{query}",
        oninput: move |event| query.set(event.value()),
      }
      ModelList {
        provider,
        query: text,
        visible,
        flags,
        ontoggle: move |(index, value)| set_one(shown, index, value),
      }
    }
  }
}

fn set_shown(mut shown: Signal<Vec<bool>>, value: bool) {
  shown.write().fill(value);
}

fn set_one(mut shown: Signal<Vec<bool>>, index: usize, value: bool) {
  if let Some(flag) = shown.write().get_mut(index) {
    *flag = value;
  }
}

#[component]
fn ModelList(
  provider: Provider,
  query: String,
  visible: Vec<usize>,
  flags: Vec<bool>,
  ontoggle: EventHandler<(usize, bool)>,
) -> Element {
  if visible.is_empty() {
    return rsx! {
      p { class: "tb-provider-empty", role: "status",
        if query.trim().is_empty() {
          "No models in this catalog."
        } else {
          "No models match “{query}”."
        }
      }
    };
  }
  rsx! {
    div { class: "tb-provider-card tb-models",
      for index in visible {
        ModelRow {
          key: "{index}",
          name: model_name(provider, index),
          id: model_id(provider, index),
          on: flag_at(&flags, index),
          ontoggle: move |value| ontoggle.call((index, value)),
        }
      }
    }
  }
}

fn model_name(provider: Provider, index: usize) -> &'static str {
  provider
    .models
    .get(index)
    .map(|model| model.name)
    .unwrap_or("")
}

fn model_id(provider: Provider, index: usize) -> &'static str {
  provider
    .models
    .get(index)
    .map(|model| model.id)
    .unwrap_or("")
}

fn flag_at(flags: &[bool], index: usize) -> bool {
  flags.get(index).copied().unwrap_or(false)
}

#[component]
fn ModelRow(name: String, id: String, on: bool, ontoggle: EventHandler<bool>) -> Element {
  rsx! {
    div { class: "tb-model",
      span { class: "tb-model-name",
        "{name}"
        span { class: "tb-model-id", "{id}" }
      }
      label { class: "tb-switch",
        span { class: "sr-only", "Show {name} in the model picker" }
        input {
          r#type: "checkbox",
          role: "switch",
          checked: on,
          onchange: move |event| ontoggle.call(event.checked()),
        }
      }
    }
  }
}

/// Default-provider control and the switch that turns this provider on.
#[component]
pub(super) fn UseBar(
  name: String,
  default_name: String,
  is_default: bool,
  can_default: bool,
  initially_on: bool,
  on_make_default: EventHandler,
) -> Element {
  let mut enabled = use_signal(|| initially_on);
  rsx! {
    section { class: "tb-provider-block", "aria-labelledby": "provider-use",
      h2 { id: "provider-use", "Use" }
      div { class: "tb-provider-card",
        div { class: "tb-card-row",
          div {
            div { "Default for new agents" }
            p { class: "tb-muted", "New agents start with {default_name}." }
          }
          DefaultButton {
            is_default,
            can_default,
            on_make_default,
          }
        }
        div { class: "tb-card-split tb-card-row",
          span { "Use {name}" }
          label { class: "tb-switch",
            span { class: "sr-only", "Use {name}" }
            input {
              r#type: "checkbox",
              role: "switch",
              checked: enabled(),
              onchange: move |event| enabled.set(event.checked()),
            }
          }
        }
      }
    }
  }
}

#[component]
fn DefaultButton(is_default: bool, can_default: bool, on_make_default: EventHandler) -> Element {
  let title = default_title(is_default, can_default);
  rsx! {
    button {
      class: "tb-provider-btn",
      r#type: "button",
      disabled: is_default || !can_default,
      title: if !title.is_empty() { title },
      onclick: move |_| on_make_default.call(()),
      if is_default {
        "Default"
      } else {
        "Make default"
      }
    }
  }
}

fn default_title(is_default: bool, can_default: bool) -> &'static str {
  if is_default {
    "Already the default."
  } else if !can_default {
    "Connect it before making it the default."
  } else {
    ""
  }
}
