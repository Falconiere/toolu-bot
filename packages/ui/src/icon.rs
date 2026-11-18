use dioxus::prelude::*;

/// Lucide-compatible line icons: 24px grid, 1.75 stroke, `currentColor`.
#[derive(Clone, Copy, PartialEq)]
pub enum IconName {
  Agents,
  Flows,
  Tools,
  Knowledge,
  Channels,
  Runs,
  Models,
  Settings,
  Plus,
  Check,
  Pause,
  X,
}

impl IconName {
  /// Inner SVG markup on the 24px grid.
  fn paths(self) -> &'static str {
    match self {
      IconName::Agents => {
        r#"<circle cx="12" cy="8" r="4"/><path d="M4 20c1.5-4 4.5-6 8-6s6.5 2 8 6"/>"#
      }
      IconName::Flows => {
        r#"<circle cx="5" cy="12" r="2"/><circle cx="19" cy="6" r="2"/><circle cx="19" cy="18" r="2"/><path d="M7 12h4l6-6M11 12l6 6"/>"#
      }
      IconName::Tools => r#"<path d="M14 4l6 6-10 10H4v-6z"/>"#,
      IconName::Knowledge => r#"<path d="M5 4h10l4 4v12H5z"/><path d="M9 12h6M9 16h6"/>"#,
      IconName::Channels => r#"<path d="M4 6h16v10H8l-4 4z"/>"#,
      IconName::Runs => r#"<path d="M4 12h4l3-7 4 14 3-7h2"/>"#,
      IconName::Models => {
        r#"<rect x="5" y="5" width="14" height="14" rx="2"/><path d="M9 1v4M15 1v4M9 19v4M15 19v4M1 9h4M1 15h4M19 9h4M19 15h4"/>"#
      }
      IconName::Settings => {
        r#"<circle cx="12" cy="12" r="3"/><path d="M12 2v3M12 19v3M2 12h3M19 12h3"/>"#
      }
      IconName::Plus => r#"<path d="M12 5v14M5 12h14"/>"#,
      IconName::Check => r#"<path d="M5 12.5l4.5 4.5L19 7.5"/>"#,
      IconName::Pause => r#"<path d="M9 5v14M15 5v14"/>"#,
      IconName::X => r#"<path d="M6 6l12 12M18 6L6 18"/>"#,
    }
  }
}

/// One icon from the set, sized and coloured by its container.
#[component]
pub fn Icon(name: IconName) -> Element {
  rsx! {
    svg {
      class: "tb-icon",
      view_box: "0 0 24 24",
      "aria-hidden": "true",
      dangerous_inner_html: name.paths(),
    }
  }
}
