use dioxus::prelude::*;

fn main() {
  // Debug builds float above other apps: dioxus-desktop `Config::new` and
  // `dx serve --always-on-top` both default to true. Opt back out here.
  #[cfg(feature = "desktop")]
  dioxus::LaunchBuilder::desktop()
    .with_cfg(
      dioxus::desktop::Config::new().with_window(
        dioxus::desktop::WindowBuilder::new()
          .with_title(dioxus::cli_config::app_title().unwrap_or_else(|| "Dioxus App".to_string()))
          .with_always_on_top(false),
      ),
    )
    .launch(App);

  #[cfg(not(feature = "desktop"))]
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    ui::Theme {}
    ui::AppShell {}
  }
}
