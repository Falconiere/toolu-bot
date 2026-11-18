//! TooluBot shared UI: design tokens, primitives and the agent-builder shell.
//! Source of truth: the TooluBot design system (https://claude.ai/artifact/7eLRjAFNEkRzFeYSF9SdFv).

mod agents;
mod icon;
mod primitives;
mod shell;
mod theme;

pub use agents::{Agent, Turn};
pub use icon::{Icon, IconName};
pub use primitives::*;
pub use shell::AppShell;
pub use theme::{Theme, MARK};
