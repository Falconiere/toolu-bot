use super::*;

#[test]
fn grok_matches_the_reference_screen() {
  assert_eq!(
    find("Grok").map(|page| page
      .provider
      .models
      .iter()
      .map(|model| model.id)
      .collect::<Vec<_>>()),
    Some(vec![
      "grok-4.7",
      "grok-4.7-build-fast",
      "grok-4.6",
      "grok-4.5"
    ])
  );
  assert_eq!(
    find("Grok").map(|page| (page.index, page.provider.version)),
    Some((Some(2), Some("v1.0.92")))
  );
  assert!(find("Cline").is_some_and(|page| page.index.is_none() && !page.provider.connected));
  assert!(find("Missing").is_none());
  assert_eq!(default_name(1), "Claude");
  assert_eq!(default_name(99), CONNECTED[0].name);
  assert_eq!(picker_line(4, 4), "4 of 4 models show in the model picker.");
  assert_eq!(accounts_line(1), "1 of 20. Each agent uses one account.");
  assert_eq!(agents_line(1), "1 agent uses it");
  assert_eq!(agents_line(0), "0 agents use it");
}

#[test]
fn used_by_line_joins_names() {
  assert_eq!(used_by_line(&[]), None);
  assert_eq!(
    used_by_line(&["Scout".into()]).as_deref(),
    Some("Used by Scout")
  );
  assert_eq!(
    used_by_line(&["Scout".into(), "Pair".into()]).as_deref(),
    Some("Used by Scout and Pair")
  );
  assert_eq!(
    used_by_line(&["Scout".into(), "Pair".into(), "Lore".into()]).as_deref(),
    Some("Used by Scout, Pair, and Lore")
  );
  let rows = [
    AccountRow {
      label: "a".into(),
      used_by: vec!["Scout".into(), "Pair".into()],
    },
    AccountRow {
      label: "b".into(),
      used_by: vec!["Pair".into(), "Lore".into()],
    },
  ];
  assert_eq!(agent_count(&rows), 3);
  assert_eq!(next_label(0, false), "you@example.com");
  assert_eq!(next_label(1, false), "you+2@example.com");
  assert_eq!(next_label(0, true), "Agents - VPS 1");
}

#[test]
fn account_edits_stop_at_the_limit() {
  let mut editable = vec![
    AccountRow {
      label: "a".into(),
      used_by: vec!["Scout".into()],
    },
    AccountRow {
      label: "b".into(),
      used_by: vec![],
    },
  ];
  remove_unused(&mut editable);
  assert_eq!(editable.len(), 1);
  remove_account(&mut editable, 5);
  assert_eq!(editable.len(), 1);
  remove_account(&mut editable, 0);
  assert!(editable.is_empty());
  add_account(&mut editable, false);
  assert_eq!(
    editable.first().map(|row| row.label.as_str()),
    Some("you@example.com")
  );
  editable.resize(
    ACCOUNT_LIMIT,
    AccountRow {
      label: "x".into(),
      used_by: vec![],
    },
  );
  add_account(&mut editable, false);
  assert_eq!(editable.len(), ACCOUNT_LIMIT);
}

#[test]
fn model_search_matches_name_or_id() {
  let models = [
    model("Grok 4.7 Fast", "grok-4.7-build-fast"),
    model("Grok 4.5", "grok-4.5"),
  ];
  assert_eq!(matching_model_indexes(&models, "fast"), vec![0]);
  assert_eq!(matching_model_indexes(&models, "4.5"), vec![1]);
  assert_eq!(matching_model_indexes(&models, "  "), vec![0, 1]);
}
