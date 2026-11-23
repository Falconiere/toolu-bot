use super::{toggle_section, Tab};

#[test]
fn section_toggle_round_trips() {
  let mut open = vec![Tab::Persona];
  toggle_section(&mut open, Tab::Brain);
  assert!(open.contains(&Tab::Brain));
  toggle_section(&mut open, Tab::Brain);
  assert!(!open.contains(&Tab::Brain));
  assert!(open.contains(&Tab::Persona));
}
