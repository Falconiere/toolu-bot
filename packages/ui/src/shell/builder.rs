//! The agent builder: header, settings tabs, persona editor.
use super::update;
use crate::*;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
  Persona,
  Brain,
  Tools,
  Memory,
  Channels,
  Guardrails,
}

impl Tab {
  const ALL: [Tab; 6] = [
    Tab::Persona,
    Tab::Brain,
    Tab::Tools,
    Tab::Memory,
    Tab::Channels,
    Tab::Guardrails,
  ];

  fn label(self) -> &'static str {
    match self {
      Tab::Persona => "Persona",
      Tab::Brain => "Brain",
      Tab::Tools => "Tools",
      Tab::Memory => "Memory",
      Tab::Channels => "Channels",
      Tab::Guardrails => "Guardrails",
    }
  }

  fn blurb(self) -> &'static str {
    match self {
            Tab::Persona => "",
            Tab::Brain => "The model this agent thinks with: local first, or any OpenAI-compatible endpoint, Anthropic, Mistral or Groq by key. Swap it without rewriting the agent.",
            Tab::Tools => "Which tools and MCP servers this agent may call, and which of them wait for approval.",
            Tab::Memory => "What the agent remembers between conversations. Stored on this machine.",
            Tab::Channels => "To go live: connect a channel, set reply rules (mentions only, rate limit, approval), then deploy.",
            Tab::Guardrails => "Posting publicly, spending money or touching files outside the workspace pauses for your approval by default.",
        }
  }
}

/// Agent header with Deploy, then the settings tabs.
#[component]
pub(super) fn Builder(agents: Signal<Vec<Agent>>, selected: usize) -> Element {
  let mut open = use_signal(|| vec![Tab::Persona]);
  let Some(a) = agents.read().get(selected).cloned() else {
    return rsx! {};
  };

  rsx! {
    main { class: "tb-builder",
      div { class: "tb-builder-head",
        div { class: "fm-row",
          Avatar { initial: a.initial, tone: a.tone, live: a.live }
          div {
            h1 { class: "display", "{a.name}" }
            div { class: "small mono", "{a.file()}" }
          }
        }
        Button {
          variant: ButtonVariant::Primary,
          onclick: move |_| {
              open.with_mut(|v| {
                  if !v.contains(&Tab::Channels) {
                      v.push(Tab::Channels);
                  }
              });
          },
          "Deploy"
        }
      }
      for t in Tab::ALL {
        Section { tab: t, open,
          if t == Tab::Persona {
            PersonaTab { agents, i: selected, agent: a.clone() }
          } else {
            div { class: "tb-empty",
              p { "{t.blurb()}" }
              p { class: "small", "Not wired up yet." }
            }
          }
        }
      }
    }
  }
}

/// Expand `tab` if collapsed, collapse it if expanded.
fn toggle_section(open: &mut Vec<Tab>, tab: Tab) {
  if let Some(i) = open.iter().position(|t| *t == tab) {
    open.remove(i);
  } else {
    open.push(tab);
  }
}

/// One collapsible settings section. Several may stay open at once.
#[component]
fn Section(tab: Tab, mut open: Signal<Vec<Tab>>, children: Element) -> Element {
  let expanded = open.read().contains(&tab);
  rsx! {
    div { class: "tb-section",
      button {
        r#type: "button",
        class: "tb-section-head heading",
        "aria-expanded": expanded,
        onclick: move |_| {
            open.with_mut(|v| toggle_section(v, tab));
        },
        "{tab.label()}"
        span { class: "tb-section-mark", "aria-hidden": "true",
          if expanded {
            "−"
          } else {
            "+"
          }
        }
      }
      if expanded {
        div { class: "tb-section-body", {children} }
      }
    }
  }
}

#[component]
fn PersonaTab(agents: Signal<Vec<Agent>>, i: usize, agent: Agent) -> Element {
  rsx! {
    div { class: "fm-stack", style: "gap: var(--space-6)",
      PersonaField { agents, i, persona: agent.persona.clone() }
      div { class: "tb-grid-2",
        Slider {
          id: "humour",
          label: "Humour",
          low: "Straight",
          high: "Spicy",
          value: agent.humour,
          onchange: move |v| update(agents, i, |a| a.humour = v),
        }
        Slider {
          id: "length",
          label: "Reply length",
          low: "Tweet",
          high: "Essay",
          value: agent.length,
          onchange: move |v| update(agents, i, |a| a.length = v),
        }
      }
      div { class: "tb-toggles",
        for (label, checked, set) in toggles(&agent) {
          Toggle {
            label,
            checked,
            onchange: move |v| update(agents, i, |a| set(a, v)),
          }
        }
      }
    }
  }
}

/// The personality prompt. A textarea can't be hydrated: SSR's `value` attribute is
/// ignored and hydration markers inside it become literal text. So the real one renders
/// after mount, when Dioxus sets `value` as a property; until then a disabled stand-in.
#[component]
fn PersonaField(agents: Signal<Vec<Agent>>, i: usize, persona: String) -> Element {
  let mut mounted = use_signal(|| false);
  use_effect(move || mounted.set(true));
  rsx! {
    Field { id: "persona", label: "Personality",
      if mounted() {
        textarea {
          id: "persona",
          class: "fm-input fm-input--mono",
          rows: "6",
          placeholder: "You are…",
          value: "{persona}",
          oninput: move |e| update(agents, i, |a| a.persona = e.value()),
        }
      } else {
        textarea {
          id: "persona",
          class: "fm-input fm-input--mono",
          rows: "6",
          disabled: true,
        }
      }
    }
  }
}

type Setter = fn(&mut Agent, bool);

/// The on/off settings under the persona, in display order.
fn toggles(a: &Agent) -> [(&'static str, bool, Setter); 3] {
  [
    ("Long-term memory (local SQLite)", a.memory, |a, v| {
      a.memory = v
    }),
    (
      "Ask before posting publicly",
      a.ask_before_posting,
      |a, v| a.ask_before_posting = v,
    ),
    ("Live web search", a.web_search, |a, v| a.web_search = v),
  ]
}

#[component]
fn Slider(
  id: String,
  label: String,
  low: String,
  high: String,
  value: u8,
  onchange: EventHandler<u8>,
) -> Element {
  rsx! {
    Field { id: id.clone(), label,
      input {
        id,
        r#type: "range",
        min: "0",
        max: "100",
        value: "{value}",
        "aria-valuetext": "{value} of 100, {low} to {high}",
        oninput: move |e| onchange.call(e.value().parse().unwrap_or(value)),
      }
      div { class: "tb-scale small", "aria-hidden": "true",
        span { "{low}" }
        span { "{high}" }
      }
    }
  }
}

#[cfg(test)]
#[path = "tests/builder.rs"]
mod tests;
