//! Design-system primitives. Class names (`fm-*`) come verbatim from the system's bundle.css.
use crate::{Icon, IconName};
use dioxus::prelude::*;

/// Button weight. One `Primary` (ember) per view.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
  Primary,
  #[default]
  Secondary,
  Ghost,
  Danger,
}

/// Triggers an action. Label is a verb, sentence case.
#[component]
pub fn Button(
  #[props(default)] variant: ButtonVariant,
  #[props(default)] small: bool,
  #[props(default)] disabled: bool,
  onclick: Option<EventHandler<MouseEvent>>,
  children: Element,
) -> Element {
  let v = match variant {
    ButtonVariant::Primary => "fm-btn--primary",
    ButtonVariant::Secondary => "fm-btn--secondary",
    ButtonVariant::Ghost => "fm-btn--ghost",
    ButtonVariant::Danger => "fm-btn--danger",
  };
  let size = if small { "fm-btn--sm" } else { "" };
  rsx! {
    button {
      r#type: "button",
      class: "fm-btn {v} {size}",
      disabled,
      onclick: move |e| {
          if let Some(h) = onclick {
              h.call(e)
          }
      },
      {children}
    }
  }
}

/// Signal colour for badges. Colour always travels with a word.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Tone {
  #[default]
  Idle,
  Online,
  Tool,
  Warn,
  Error,
  Brand,
}

/// Short status label with a leading dot (`plain` drops the dot).
#[component]
pub fn Badge(
  #[props(default)] tone: Tone,
  #[props(default)] plain: bool,
  children: Element,
) -> Element {
  let t = match tone {
    Tone::Idle => "",
    Tone::Online => "fm-badge--online",
    Tone::Tool => "fm-badge--tool",
    Tone::Warn => "fm-badge--warn",
    Tone::Error => "fm-badge--error",
    Tone::Brand => "fm-badge--brand",
  };
  let p = if plain { "fm-badge--plain" } else { "" };
  rsx! {
    span { class: "fm-badge {t} {p}", {children} }
  }
}

/// Avatar tint. Ember is reserved for the agent running right now.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarTone {
  #[default]
  Neutral,
  Ember,
  Moss,
  Sky,
}

/// An agent's initial on a tinted square; `live` adds the slow ember pulse.
#[component]
pub fn Avatar(
  initial: char,
  #[props(default)] tone: AvatarTone,
  #[props(default)] small: bool,
  #[props(default)] live: bool,
) -> Element {
  let t = match tone {
    AvatarTone::Neutral => "",
    AvatarTone::Ember => "fm-avatar--ember",
    AvatarTone::Moss => "fm-avatar--moss",
    AvatarTone::Sky => "fm-avatar--sky",
  };
  let s = if small { "fm-avatar--sm" } else { "" };
  let l = if live { "fm-avatar--live" } else { "" };
  rsx! {
    div { class: "fm-avatar {t} {s} {l}", "aria-hidden": "true", "{initial}" }
  }
}

/// Label, control and help line. The child control must carry `id`.
#[component]
pub fn Field(
  id: String,
  label: String,
  help: Option<String>,
  #[props(default)] error: bool,
  children: Element,
) -> Element {
  let help_id = format!("{id}-help");
  rsx! {
    div { class: if error { "fm-field fm-field--error" } else { "fm-field" },
      label { r#for: "{id}", "{label}" }
      {children}
      if let Some(help) = help {
        span { id: "{help_id}", class: "fm-help", "{help}" }
      }
    }
  }
}

/// Immediate on/off setting: label left, switch right, full-width row.
#[component]
pub fn Toggle(label: String, checked: bool, onchange: EventHandler<bool>) -> Element {
  rsx! {
    label { class: "fm-row fm-toggle-row",
      span { "{label}" }
      input {
        r#type: "checkbox",
        role: "switch",
        class: "fm-toggle",
        checked,
        onchange: move |e| onchange.call(e.checked()),
      }
    }
  }
}

/// Who said a chat turn.
#[derive(Clone, Copy, PartialEq)]
pub enum Speaker {
  User,
  Agent { initial: char, tone: AvatarTone },
}

/// One turn in a test chat or channel transcript. `meta` is agent · time · tokens.
#[component]
pub fn ChatMessage(from: Speaker, text: String, meta: Option<String>) -> Element {
  match from {
    Speaker::User => rsx! {
      div { class: "fm-msg fm-msg--user",
        div { class: "fm-msg__bubble", "{text}" }
      }
    },
    Speaker::Agent { initial, tone } => rsx! {
      div { class: "fm-msg",
        Avatar { initial, tone, small: true }
        div {
          div { class: "fm-msg__bubble", "{text}" }
          if let Some(meta) = meta {
            div { class: "fm-msg__time", "{meta}" }
          }
        }
      }
    },
  }
}

/// Outcome of a tool or MCP call.
#[derive(Clone, Copy, PartialEq)]
pub enum ToolStatus {
  Done,
  NeedsApproval,
  Failed,
}

/// A tool call inline in a conversation. Done calls start collapsed; approvals
/// stay open and take their Approve / Edit / Deny buttons as children.
#[component]
pub fn ToolCall(
  name: String,
  status: ToolStatus,
  note: Option<String>,
  body: Option<String>,
  children: Element,
) -> Element {
  let (class, icon, suffix) = match status {
    ToolStatus::Done => ("fm-tool", IconName::Check, ""),
    ToolStatus::NeedsApproval => (
      "fm-tool fm-tool--approve",
      IconName::Pause,
      " · needs approval",
    ),
    ToolStatus::Failed => ("fm-tool fm-tool--error", IconName::X, ""),
  };
  let bare = if body.is_none() && status == ToolStatus::Done {
    "fm-tool--bare"
  } else {
    ""
  };
  rsx! {
    details { class: "{class} {bare}", open: status != ToolStatus::Done,
      summary { class: "fm-tool__head",
        Icon { name: icon }
        "{name}{suffix}"
        if let Some(note) = note {
          span { class: "fm-tool__dur", "{note}" }
        }
      }
      if let Some(body) = body {
        pre { class: "fm-tool__body", "{body}" }
      }
      if status == ToolStatus::NeedsApproval {
        div { class: "fm-tool__actions", {children} }
      }
    }
  }
}

/// One agent in the library grid: avatar, name, status, purpose, model + tool chips.
#[component]
pub fn AgentCard(agent: crate::Agent, #[props(default)] selected: bool) -> Element {
  // At most three chips: the model plus tools, collapsing the rest into +N.
  let shown = if agent.tools.len() <= 2 {
    agent.tools.len()
  } else {
    1
  };
  let extra = agent.tools.len() - shown;
  rsx! {
    div { class: if selected { "fm-agent fm-agent--selected" } else { "fm-agent" },
      div { class: "fm-agent__head",
        Avatar {
          initial: agent.initial,
          tone: agent.tone,
          live: agent.live,
        }
        div {
          div { class: "fm-agent__name", "{agent.name}" }
          Badge { tone: if agent.live { Tone::Online } else { Tone::Idle }, "{agent.status}" }
        }
      }
      p { class: "fm-agent__desc", "{agent.purpose}" }
      div { class: "fm-agent__meta",
        span { class: "fm-chip", "{agent.model}" }
        for tool in agent.tools.iter().take(shown) {
          span { class: "fm-chip", "{tool}" }
        }
        if extra > 0 {
          span { class: "fm-chip", "+{extra}" }
        }
      }
    }
  }
}
