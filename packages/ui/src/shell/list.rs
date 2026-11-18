//! The agent library column.
use crate::*;
use dioxus::prelude::*;

/// Searchable list of every agent; selecting one opens it in the builder.
#[component]
pub(super) fn AgentList(agents: Signal<Vec<Agent>>, mut selected: Signal<usize>) -> Element {
  let mut query = use_signal(String::new);
  let count = agents.read().len();
  let matches = filter(&agents.read(), &query().to_lowercase());
  let new_agent = move |_| add_blank(agents, selected);

  rsx! {
    section { class: "tb-list", "aria-label": "Agents",
      div { class: "fm-row tb-list-head",
        h2 { class: "fm-label", "Agents · {count}" }
        Button {
          variant: ButtonVariant::Ghost,
          small: true,
          onclick: new_agent,
          Icon { name: IconName::Plus }
          "New"
        }
      }
      input {
        class: "fm-input",
        r#type: "search",
        placeholder: "Search agents",
        "aria-label": "Search agents",
        value: "{query}",
        oninput: move |e| query.set(e.value()),
      }
      if matches.is_empty() {
        p { class: "small", "No agent called “{query}”." }
      }
      for (i, a) in matches {
        AgentRow {
          key: "{i}",
          agent: a,
          current: i == selected(),
          onselect: move |_| selected.set(i),
        }
      }
      p { class: "small tb-list-foot",
        "Agents are plain files in "
        code { "~/TooluBot/agents/" }
      }
    }
  }
}

#[component]
fn AgentRow(agent: Agent, current: bool, onselect: EventHandler<()>) -> Element {
  rsx! {
    button {
      r#type: "button",
      class: "tb-agent-row",
      "aria-current": if current { "true" },
      onclick: move |_| onselect.call(()),
      Avatar {
        initial: agent.initial,
        tone: agent.tone,
        small: true,
        live: agent.live,
      }
      div { class: "tb-agent-text",
        div { class: "tb-agent-name", "{agent.name}" }
        div { class: "small", "{agent.channels}" }
      }
      if agent.live {
        Badge { tone: Tone::Online, plain: true, "Live" }
      }
    }
  }
}

/// Agents whose name contains `q` (lowercase), with their index.
fn filter(agents: &[Agent], q: &str) -> Vec<(usize, Agent)> {
  agents
    .iter()
    .cloned()
    .enumerate()
    .filter(|(_, a)| a.name.to_lowercase().contains(q))
    .collect()
}

/// Add a Blank-starter agent and select it.
fn add_blank(mut agents: Signal<Vec<Agent>>, mut selected: Signal<usize>) {
  let n = agents
    .read()
    .iter()
    .filter(|a| a.name.starts_with("Untitled"))
    .count();
  agents.write().push(Agent::blank(n));
  selected.set(agents.read().len() - 1);
}
