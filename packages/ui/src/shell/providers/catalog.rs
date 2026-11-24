//! Illustrative provider catalogs. Sign-in and model lists preview the reference
//! screen until a runtime exists. Grok's models follow that screen.

use dioxus::prelude::*;

/// A model host shown in the provider list and its inner page.
#[derive(Clone, Copy, PartialEq)]
pub(super) struct Provider {
  pub(super) name: &'static str,
  pub(super) status: &'static str,
  pub(super) logo: Asset,
  pub(super) connected: bool,
  /// Account labels are hosts ("Agents - VPS") rather than emails.
  pub(super) hosted: bool,
  pub(super) version: Option<&'static str>,
  pub(super) accounts: &'static [Account],
  pub(super) models: &'static [Model],
}

/// One signed-in account on a provider, and the agents that use it.
#[derive(Clone, Copy, PartialEq)]
pub(super) struct Account {
  pub(super) label: &'static str,
  pub(super) used_by: &'static [&'static str],
}

/// A model the provider can show in the picker.
#[derive(Clone, Copy, PartialEq)]
pub(super) struct Model {
  pub(super) name: &'static str,
  pub(super) id: &'static str,
}

/// An account edited on the inner page. Starts as a copy of [`Account`].
#[derive(Clone, PartialEq)]
pub(super) struct AccountRow {
  pub(super) label: String,
  pub(super) used_by: Vec<String>,
}

/// A catalog hit. `index` is set when the provider can be the default.
#[derive(Clone, Copy, PartialEq)]
pub(super) struct Found {
  pub(super) provider: Provider,
  pub(super) index: Option<usize>,
}

/// How many accounts one provider can hold.
pub(super) const ACCOUNT_LIMIT: usize = 20;

const fn model(name: &'static str, id: &'static str) -> Model {
  Model { name, id }
}

/// Providers that already have an account in the preview.
pub(super) const CONNECTED: [Provider; 8] = [
  Provider {
    name: "ChatGPT",
    status: "you@example.com",
    logo: asset!("/assets/providers/chatgpt.svg"),
    connected: true,
    hosted: false,
    version: Some("v1.4.2"),
    accounts: &[Account {
      label: "you@example.com",
      used_by: &["Scout", "Pair"],
    }],
    models: &[
      model("GPT-4.1", "gpt-4.1"),
      model("GPT-4.1 mini", "gpt-4.1-mini"),
      model("o4-mini", "o4-mini"),
    ],
  },
  Provider {
    name: "Claude",
    status: "you@example.com",
    logo: asset!("/assets/providers/claude.svg"),
    connected: true,
    hosted: false,
    version: Some("v1.2.0"),
    accounts: &[Account {
      label: "you@example.com",
      used_by: &["Dispatch", "Lore"],
    }],
    models: &[
      model("Claude Opus 4.5", "claude-opus-4-5"),
      model("Claude Sonnet 4.5", "claude-sonnet-4-5"),
      model("Claude Haiku 4.5", "claude-haiku-4-5"),
    ],
  },
  Provider {
    name: "Grok",
    status: "you@example.com",
    logo: asset!("/assets/providers/grok.svg"),
    connected: true,
    hosted: false,
    version: Some("v1.0.92"),
    accounts: &[Account {
      label: "you@example.com",
      used_by: &[
        "Herald",
        "Digest",
        "Archivist",
        "Inbox",
        "Warden",
        "Scribe",
        "Courier",
        "Beacon",
        "Quill",
        "Ledger",
      ],
    }],
    models: &[
      model("Grok 4.7", "grok-4.7"),
      model("Grok 4.7 Fast", "grok-4.7-build-fast"),
      model("Grok 4.6", "grok-4.6"),
      model("Grok 4.5", "grok-4.5"),
    ],
  },
  Provider {
    name: "OpenCode",
    status: "Runs on Agents - VPS",
    logo: asset!("/assets/providers/opencode.svg"),
    connected: true,
    hosted: true,
    version: Some("v0.9.1"),
    accounts: &[Account {
      label: "Runs on Agents - VPS",
      used_by: &["Porter"],
    }],
    models: &[
      model("OpenCode Go", "opencode-go"),
      model("OpenCode Spark", "opencode-spark"),
    ],
  },
  Provider {
    name: "Gemini",
    status: "you@example.com",
    logo: asset!("/assets/providers/gemini.svg"),
    connected: true,
    hosted: false,
    version: Some("v1.1.0"),
    accounts: &[Account {
      label: "you@example.com",
      used_by: &["Atlas"],
    }],
    models: &[
      model("Gemini 2.5 Pro", "gemini-2.5-pro"),
      model("Gemini 2.5 Flash", "gemini-2.5-flash"),
    ],
  },
  Provider {
    name: "Cursor",
    status: "Runs on Agents - VPS",
    logo: asset!("/assets/providers/cursor.svg"),
    connected: true,
    hosted: true,
    version: Some("v0.8.4"),
    accounts: &[Account {
      label: "Runs on Agents - VPS",
      used_by: &["Nock"],
    }],
    models: &[
      model("Cursor composer", "composer"),
      model("Cursor small", "composer-small"),
    ],
  },
  Provider {
    name: "Muse",
    status: "you@example.com",
    logo: asset!("/assets/providers/terminal.svg"),
    connected: true,
    hosted: false,
    version: Some("v0.6.0"),
    accounts: &[Account {
      label: "you@example.com",
      used_by: &["Fable"],
    }],
    models: &[model("Muse", "muse"), model("Muse small", "muse-small")],
  },
  Provider {
    name: "OpenRouter",
    status: "Runs on Agents - VPS",
    logo: asset!("/assets/providers/openrouter.svg"),
    connected: true,
    hosted: true,
    version: Some("v1.3.1"),
    accounts: &[Account {
      label: "Runs on Agents - VPS",
      used_by: &["Route", "Switch"],
    }],
    models: &[
      model("Claude Sonnet", "anthropic/claude-sonnet"),
      model("GPT-4.1", "openai/gpt-4.1"),
      model("Grok 4.7", "x-ai/grok-4.7"),
      model("Gemini Flash", "google/gemini-flash"),
      model("Llama 3.3 70B", "meta/llama-3.3-70b"),
    ],
  },
];

/// Providers that are not signed in or not installed.
pub(super) const OTHER: [Provider; 2] = [
  Provider {
    name: "Cline",
    status: "Not downloaded",
    logo: asset!("/assets/providers/cline.svg"),
    connected: false,
    hosted: false,
    version: None,
    accounts: &[],
    models: &[model("Cline default", "cline")],
  },
  Provider {
    name: "Pi",
    status: "Not connected",
    logo: asset!("/assets/providers/terminal.svg"),
    connected: false,
    hosted: false,
    version: None,
    accounts: &[],
    models: &[model("Pi", "pi"), model("Pi infill", "pi-infill")],
  },
];

/// Look up a provider by its list name.
pub(super) fn find(name: &str) -> Option<Found> {
  if let Some(index) = CONNECTED.iter().position(|provider| provider.name == name) {
    return Some(Found {
      provider: CONNECTED[index],
      index: Some(index),
    });
  }
  OTHER
    .iter()
    .find(|provider| provider.name == name)
    .copied()
    .map(|provider| Found {
      provider,
      index: None,
    })
}

/// Name of the default provider. Out-of-range indexes fall back to the first.
pub(super) fn default_name(index: usize) -> &'static str {
  CONNECTED
    .get(index)
    .or_else(|| CONNECTED.first())
    .map(|provider| provider.name)
    .unwrap_or("a provider")
}

/// Providers whose name or status contains `query` (already lowercased).
pub(super) fn matching(providers: &[Provider], query: &str) -> Vec<Provider> {
  providers
    .iter()
    .copied()
    .filter(|provider| {
      provider.name.to_lowercase().contains(query) || provider.status.to_lowercase().contains(query)
    })
    .collect()
}

/// Copy a provider's accounts so the page can add and remove them.
pub(super) fn owned_accounts(provider: Provider) -> Vec<AccountRow> {
  provider
    .accounts
    .iter()
    .map(|account| AccountRow {
      label: account.label.to_string(),
      used_by: account
        .used_by
        .iter()
        .map(|name| (*name).to_string())
        .collect(),
    })
    .collect()
}

/// "1 of 20. Each agent uses one account."
pub(super) fn accounts_line(count: usize) -> String {
  format!("{count} of {ACCOUNT_LIMIT}. Each agent uses one account.")
}

/// "1 agent uses it" or "4 agents use it".
pub(super) fn agents_line(count: usize) -> String {
  if count == 1 {
    "1 agent uses it".to_string()
  } else {
    format!("{count} agents use it")
  }
}

/// "4 of 4 models show in the model picker."
pub(super) fn picker_line(shown: usize, total: usize) -> String {
  format!("{shown} of {total} models show in the model picker.")
}

/// "Used by A", "Used by A and B", or "Used by A, B, and C".
pub(super) fn used_by_line(names: &[String]) -> Option<String> {
  let (last, rest) = names.split_last()?;
  let line = if rest.is_empty() {
    format!("Used by {last}")
  } else if let [only] = rest {
    format!("Used by {only} and {last}")
  } else {
    format!("Used by {}, and {last}", rest.join(", "))
  };
  Some(line)
}

/// Agents using this provider, counted once across accounts.
pub(super) fn agent_count(rows: &[AccountRow]) -> usize {
  let mut names: Vec<&str> = rows
    .iter()
    .flat_map(|row| row.used_by.iter().map(String::as_str))
    .collect();
  names.sort_unstable();
  names.dedup();
  names.len()
}

/// Label for the next account added on the page.
pub(super) fn next_label(count: usize, hosted: bool) -> String {
  let n = count + 1;
  if hosted {
    format!("Agents - VPS {n}")
  } else if count == 0 {
    "you@example.com".to_string()
  } else {
    format!("you+{n}@example.com")
  }
}

/// Append an account until [`ACCOUNT_LIMIT`]. Labels stay unique after a removal.
pub(super) fn add_account(rows: &mut Vec<AccountRow>, hosted: bool) {
  if rows.len() >= ACCOUNT_LIMIT {
    return;
  }
  rows.push(AccountRow {
    label: fresh_label(rows, hosted),
    used_by: Vec::new(),
  });
}

fn fresh_label(rows: &[AccountRow], hosted: bool) -> String {
  (0..ACCOUNT_LIMIT)
    .map(|offset| next_label(rows.len() + offset, hosted))
    .find(|label| rows.iter().all(|row| row.label != *label))
    .unwrap_or_else(|| next_label(rows.len() + ACCOUNT_LIMIT, hosted))
}

/// Drop one account. An out-of-range index does nothing.
pub(super) fn remove_account(rows: &mut Vec<AccountRow>, index: usize) {
  if index < rows.len() {
    rows.remove(index);
  }
}

/// Drop accounts that no agent uses.
pub(super) fn remove_unused(rows: &mut Vec<AccountRow>) {
  rows.retain(|row| !row.used_by.is_empty());
}

/// Indexes of models whose name or id contains the query.
pub(super) fn matching_model_indexes(models: &[Model], query: &str) -> Vec<usize> {
  let q = query.trim().to_lowercase();
  models
    .iter()
    .enumerate()
    .filter(|(_, model)| {
      q.is_empty() || model.name.to_lowercase().contains(&q) || model.id.to_lowercase().contains(&q)
    })
    .map(|(index, _)| index)
    .collect()
}

#[cfg(test)]
#[path = "tests/catalog.rs"]
mod tests;
