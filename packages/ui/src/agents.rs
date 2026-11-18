use crate::{AvatarTone, ToolStatus};

/// One turn in an agent's test chat.
#[derive(Clone, PartialEq)]
pub enum Turn {
  User(String),
  Agent {
    text: String,
    meta: String,
  },
  Tool {
    name: String,
    status: ToolStatus,
    note: Option<String>,
    body: Option<String>,
  },
}

/// An agent as the builder edits it.
#[derive(Clone, PartialEq)]
pub struct Agent {
  pub name: String,
  pub initial: char,
  pub tone: AvatarTone,
  /// Where it is live, e.g. "X · Discord".
  pub channels: String,
  pub live: bool,
  pub status: String,
  pub purpose: String,
  pub model: String,
  pub tools: Vec<String>,
  pub persona: String,
  /// 0 = straight, 100 = spicy.
  pub humour: u8,
  /// 0 = tweet, 100 = essay.
  pub length: u8,
  pub memory: bool,
  pub ask_before_posting: bool,
  pub web_search: bool,
  pub chat: Vec<Turn>,
}

impl Agent {
  /// A fresh agent from the Blank starter.
  pub fn blank(n: usize) -> Self {
    Agent {
      name: if n == 0 {
        "Untitled".into()
      } else {
        format!("Untitled {}", n + 1)
      },
      initial: 'U',
      tone: AvatarTone::Neutral,
      channels: "Local only".into(),
      live: false,
      status: "Draft".into(),
      purpose: String::new(),
      model: "llama-3.3-70b".into(),
      tools: vec![],
      persona: String::new(),
      humour: 50,
      length: 50,
      memory: false,
      ask_before_posting: true,
      web_search: false,
      chat: vec![],
    }
  }

  /// File name on disk, from the agent's name.
  pub fn file(&self) -> String {
    format!("{}.agent.yaml", self.name.to_lowercase().replace(' ', "-"))
  }

  // shortcut: static demo agents from the design system's AppShell screen;
  // replace with agents loaded from ~/TooluBot/agents/*.agent.yaml once the core exists.
  /// The four demo agents shown in the builder.
  pub fn demo() -> Vec<Agent> {
    vec![scout(), dispatch(), pair(), lore()]
  }
}

fn strings(v: &[&str]) -> Vec<String> {
  v.iter().map(|t| t.to_string()).collect()
}

fn scout() -> Agent {
  Agent {
        name: "Scout".into(),
        initial: 'S',
        tone: AvatarTone::Ember,
        channels: "X · Discord".into(),
        live: true,
        status: "Online · X, Discord".into(),
        purpose: "Witty research bot that answers mentions with sourced replies.".into(),
        model: "llama-3.3-70b".into(),
        tools: strings(&["web.search", "web.fetch", "x.post"]),
        persona: "You are Scout: dry, curious, a little irreverent. You answer questions people ask you on X with short, sourced replies. You enjoy being right but say so when you aren't sure. Never invent numbers.".into(),
        humour: 70,
        length: 30,
        memory: true,
        ask_before_posting: true,
        web_search: true,
        chat: vec![
            Turn::User("is the new open model actually better than last month's?".into()),
            Turn::Tool {
                name: "web.search".into(),
                status: ToolStatus::Done,
                note: Some("1.4s".into()),
                body: Some("{ \"q\": \"open weights model release november\" }  → 8 results".into()),
            },
            Turn::Agent {
                text: "On reasoning benchmarks, yes — by a little. On vibes, the jury's still arguing. Two sources below.".into(),
                meta: "Scout · 14:02 · 812 tokens · 2.1s".into(),
            },
            Turn::Tool {
                name: "x.post".into(),
                status: ToolStatus::NeedsApproval,
                note: None,
                body: Some("\"Two new open releases this week, and one licence quietly got friendlier. Thread ↓\"".into()),
            },
        ],
    }
}

fn dispatch() -> Agent {
  Agent {
        name: "Dispatch".into(),
        initial: 'D',
        tone: AvatarTone::Sky,
        channels: "Slack · daily 8:00".into(),
        status: "Idle".into(),
        purpose: "Summarises the team inbox every morning into Slack.".into(),
        model: "qwen-2.5-32b".into(),
        tools: strings(&["gmail", "slack.post"]),
        persona: "You are Dispatch. Every morning at 8:00, read the team inbox and post a five-bullet summary to #ops. Flag anything that needs a reply today.".into(),
        humour: 15,
        length: 45,
        memory: true,
        web_search: false,
        ..Agent::blank(0)
    }
}

fn pair() -> Agent {
  Agent {
        name: "Pair".into(),
        initial: 'P',
        tone: AvatarTone::Moss,
        channels: "Local only".into(),
        status: "Idle".into(),
        purpose: "Rubber-duck for code review, runs only on this machine.".into(),
        model: "qwen-2.5-coder-14b".into(),
        tools: strings(&["fs.read", "code.sandbox"]),
        persona: "You are Pair, a patient senior engineer. Ask one question at a time. Point at lines, not vibes.".into(),
        humour: 30,
        length: 60,
        memory: false,
        ..Agent::blank(0)
    }
}

fn lore() -> Agent {
  Agent {
    name: "Lore".into(),
    initial: 'L',
    channels: "Telegram · draft".into(),
    purpose: "Answers questions about the community wiki on Telegram.".into(),
    tools: strings(&["knowledge.search"]),
    persona: "You are Lore, keeper of the community wiki. Quote the page you used.".into(),
    ..Agent::blank(0)
  }
}
