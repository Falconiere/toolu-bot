# UI

Every TooluBot component, style, font and screen, shared by the web, desktop and mobile apps. Built from the TooluBot design system (https://claude.ai/artifact/7eLRjAFNEkRzFeYSF9SdFv).

- `assets/`: design tokens (`toolubot.css`), the design system's component styles, local fonts and the mark.
- `src/theme.rs`: `Theme` loads the tokens, styles and fonts. Render it once at the app root.

New UI goes here so every platform gets it.

The agent screen puts test chat in the flexible center column and settings in a 400px right rail. The rail narrows to 360px at viewport widths of 1180px or less. At 900px or less, chat and settings stack with chat first.
