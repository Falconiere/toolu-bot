use dioxus::prelude::*;

/// The TooluBot mark (ember tile, eye, baseline). App icon and favicon.
pub const MARK: Asset = asset!("/assets/toolubot-mark.svg");

const TOKENS_CSS: Asset = asset!("/assets/toolubot.css");
// Loads after TOKENS_CSS: equal-specificity bundle rules win over toolubot.css.
const BUNDLE_CSS: Asset = asset!("/assets/bundle.css");
// Variable fonts, Latin subset, shipped with the app so it works offline (SIL OFL 1.1).
// shortcut: Latin only; add the latin-ext / cyrillic subsets when the UI is localised.
const INTER: Asset = asset!("/assets/fonts/inter.woff2");
const SPACE_GROTESK: Asset = asset!("/assets/fonts/space-grotesk.woff2");
const JETBRAINS_MONO: Asset = asset!("/assets/fonts/jetbrains-mono.woff2");

/// Loads tokens, component styles and fonts. Render once at the app root.
#[component]
pub fn Theme() -> Element {
  let faces = format!(
        "@font-face{{font-family:Inter;src:url({INTER}) format('woff2');font-weight:100 900;font-display:swap}}\
         @font-face{{font-family:'Space Grotesk';src:url({SPACE_GROTESK}) format('woff2');font-weight:300 700;font-display:swap}}\
         @font-face{{font-family:'JetBrains Mono';src:url({JETBRAINS_MONO}) format('woff2');font-weight:100 800;font-display:swap}}"
    );
  rsx! {
    document::Style { "{faces}" }
    document::Stylesheet { href: TOKENS_CSS }
    document::Stylesheet { href: BUNDLE_CSS }
  }
}
