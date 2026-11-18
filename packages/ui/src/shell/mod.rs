//! The main window: nav rail · agent list · builder · live test chat.
use crate::*;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Section {
  Agents,
  Flows,
  Tools,
  Knowledge,
  Channels,
  Runs,
  Models,
  Settings,
}

impl Section {
  const MAIN: [Section; 7] = [
    Section::Agents,
    Section::Flows,
    Section::Tools,
    Section::Knowledge,
    Section::Channels,
    Section::Runs,
    Section::Models,
  ];

  fn label(self) -> &'static str {
    match self {
      Section::Agents => "Agents",
      Section::Flows => "Flows",
      Section::Tools => "Tools & MCP",
      Section::Knowledge => "Knowledge",
      Section::Channels => "Channels",
      Section::Runs => "Runs",
      Section::Models => "Models",
      Section::Settings => "Settings",
    }
  }

  fn icon(self) -> IconName {
    match self {
      Section::Agents => IconName::Agents,
      Section::Flows => IconName::Flows,
      Section::Tools => IconName::Tools,
      Section::Knowledge => IconName::Knowledge,
      Section::Channels => IconName::Channels,
      Section::Runs => IconName::Runs,
      Section::Models => IconName::Models,
      Section::Settings => IconName::Settings,
    }
  }

  fn blurb(self) -> &'static str {
    match self {
            Section::Agents => "Your agents and the builder.",
            Section::Flows => "A visual canvas — trigger, agent, tool or condition, action — for scheduled and event-driven bots.",
            Section::Tools => "Built-in tools (web search, fetch, code sandbox, image gen) and any MCP server, added by URL or command.",
            Section::Knowledge => "Drop folders, PDFs and sites; they're indexed locally for retrieval.",
            Section::Channels => "Connect X, Discord, Telegram, Slack, Matrix, a webhook or a local HTTP API.",
            Section::Runs => "Every conversation and flow run: inputs, tool calls, outputs, tokens and cost. Replay any of them.",
            Section::Models => "Pull and fit local models, and keep remote keys in the OS keychain.",
            Section::Settings => "Preferences for the app itself.",
        }
  }
}

/// The TooluBot main window.
#[component]
pub fn AppShell() -> Element {
  let agents = use_signal(Agent::demo);
  let selected = use_signal(|| 0usize);
  let section = use_signal(|| Section::Agents);
  let name = agents
    .read()
    .get(selected())
    .map(|a| a.name.clone())
    .unwrap_or_default();

  rsx! {
    document::Title { "TooluBot — {name}" }
    div { class: "tb-shell",
      Rail { section }
      if section() != Section::Agents {
        main { class: "tb-section-empty",
          div { class: "tb-empty",
            h1 { class: "display", "{section().label()}" }
            p { "{section().blurb()}" }
            p { class: "small", "Not wired up yet. Agents is the working area for now." }
          }
        }
      }
    }
  }
}

#[component]
fn Rail(section: Signal<Section>) -> Element {
  rsx! {
    nav { class: "fm-rail", "aria-label": "Main",
      div { class: "tb-brand",
        img { src: MARK, alt: "" }
        span { "TooluBot" }
      }
      for s in Section::MAIN {
        NavItem { s, section }
      }
      div { class: "tb-rail-foot",
        Badge { tone: Tone::Brand, "Ollama · local" }
        NavItem { s: Section::Settings, section }
      }
    }
  }
}

#[component]
fn NavItem(s: Section, mut section: Signal<Section>) -> Element {
  let active = section() == s;
  rsx! {
    button {
      r#type: "button",
      class: if active { "fm-nav-item fm-nav-item--active" } else { "fm-nav-item" },
      "aria-current": if active { "page" },
      title: s.label(),
      onclick: move |_| section.set(s),
      Icon { name: s.icon() }
      span { "{s.label()}" }
    }
  }
}
