use dioxus::prelude::*;

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    ui::Theme {}
    ui::AppShell {}
  }
}
