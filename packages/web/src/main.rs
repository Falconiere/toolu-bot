use dioxus::prelude::*;

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    document::Link { rel: "icon", href: ui::MARK }
    ui::Theme {}
    ui::AppShell {}
  }
}
