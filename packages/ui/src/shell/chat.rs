//! The live test chat beside the builder.
use super::update;
use crate::*;
use dioxus::prelude::*;

/// Talks to the selected agent; tool calls and approvals appear inline.
#[component]
pub(super) fn TestChat(agents: Signal<Vec<Agent>>, selected: usize) -> Element {
  let Some(a) = agents.read().get(selected).cloned() else {
    return rsx! {};
  };
  let speaker = Speaker::Agent {
    initial: a.initial,
    tone: a.tone,
  };
  rsx! {
    aside { class: "tb-chat", "aria-label": "Test chat",
      div { class: "tb-chat-head",
        h2 { class: "heading", "Test chat" }
        span { class: "fm-chip", "{a.model}" }
      }
      div { class: "fm-chat", role: "log", "aria-live": "polite",
        if a.chat.is_empty() {
          div { class: "tb-empty",
            h3 { class: "heading", "Say hi to {a.name}." }
            p { class: "small", "Replies show tokens, cost and every tool call." }
          }
        }
        for (n, turn) in a.chat.into_iter().enumerate() {
          TurnView {
            key: "{selected}-{n}",
            turn,
            speaker,
            agents,
            i: selected,
            n,
          }
        }
      }
      Composer { agents, selected, name: a.name }
    }
  }
}

#[component]
fn Composer(agents: Signal<Vec<Agent>>, selected: usize, name: String) -> Element {
  let mut draft = use_signal(String::new);
  // shortcut: no model runtime yet, so every message gets the honest "not connected" error.
  let mut send = move || {
    let text = draft().trim().to_string();
    if text.is_empty() {
      return;
    }
    update(agents, selected, |a| {
      a.chat.push(Turn::User(text));
      a.chat.push(Turn::Tool {
        name: "model.generate".into(),
        status: ToolStatus::Failed,
        note: Some("not connected".into()),
        body: Some("Couldn't reach Ollama at localhost:11434. Start it, then retry.".into()),
      });
    });
    draft.set(String::new());
  };

  rsx! {
    div { class: "tb-chat-input",
      input {
        class: "fm-input",
        placeholder: "Message {name}…",
        "aria-label": "Message {name}",
        value: "{draft}",
        oninput: move |e| draft.set(e.value()),
        onkeydown: move |e| {
            if e.key() == Key::Enter {
                send()
            }
        },
      }
    }
  }
}

#[component]
fn TurnView(
  turn: Turn,
  speaker: Speaker,
  agents: Signal<Vec<Agent>>,
  i: usize,
  n: usize,
) -> Element {
  match turn {
    Turn::User(text) => rsx! {
      ChatMessage { from: Speaker::User, text }
    },
    Turn::Agent { text, meta } => rsx! {
      ChatMessage { from: speaker, text, meta }
    },
    Turn::Tool {
      name,
      status,
      note,
      body,
    } => rsx! {
      ToolCall {
        name,
        status,
        note,
        body,
        Button {
          small: true,
          onclick: move |_| resolve(agents, i, n, ToolStatus::Done, "approved by you"),
          "Approve"
        }
        Button {
          variant: ButtonVariant::Ghost,
          small: true,
          onclick: move |_| resolve(agents, i, n, ToolStatus::Failed, "denied by you"),
          "Deny"
        }
      }
    },
  }
}

/// Settle an approval: mark tool turn `n` of agent `i` with its outcome.
fn resolve(agents: Signal<Vec<Agent>>, i: usize, n: usize, to: ToolStatus, why: &'static str) {
  update(agents, i, |a| {
    if let Some(Turn::Tool { status, note, .. }) = a.chat.get_mut(n) {
      *status = to;
      *note = Some(why.into());
    }
  })
}
