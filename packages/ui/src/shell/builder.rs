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
  let mut tab = use_signal(|| Tab::Persona);
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
          onclick: move |_| tab.set(Tab::Channels),
          "Deploy"
        }
      }
      TabBar { tab }
      div { role: "tabpanel", "aria-label": tab().label(),
        if tab() == Tab::Persona {
          PersonaTab { agents, i: selected, agent: a }
        } else {
          div { class: "tb-empty",
            h2 { class: "title", "{tab().label()}" }
            p { "{tab().blurb()}" }
            p { class: "small", "Not wired up yet." }
          }
        }
      }
    }
  }
}

#[component]
fn PersonaTab(agents: Signal<Vec<Agent>>, i: usize, agent: Agent) -> Element {
  rsx! {
    div { class: "fm-stack", style: "gap: var(--space-6)",
      Field { id: "persona", label: "Personality",
        textarea {
          id: "persona",
          class: "fm-input fm-input--mono",
          rows: "6",
          placeholder: "You are…",
          value: "{agent.persona}",
          oninput: move |e| update(agents, i, |a| a.persona = e.value()),
          // SSR writes `value` as an attribute, which a textarea ignores; the text node seeds it.
          "{agent.persona}"
        }
      }
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
fn TabBar(mut tab: Signal<Tab>) -> Element {
  rsx! {
    div {
      class: "fm-tabs tb-tabs",
      role: "tablist",
      "aria-label": "Agent settings",
      for t in Tab::ALL {
        button {
          r#type: "button",
          role: "tab",
          class: if tab() == t { "fm-tab fm-tab--active" } else { "fm-tab" },
          "aria-selected": tab() == t,
          onclick: move |_| tab.set(t),
          "{t.label()}"
        }
      }
    }
  }
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
