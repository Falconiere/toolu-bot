# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

One design language shared across every target. Desktop (macOS, Windows, Linux), web and mobile all render the same Dioxus UI from `packages/ui`, through a webview or the browser. Do not fork the look per OS. Adapt layout to screen size, not to platform conventions.

## Users

Three audiences, weighted equally; design for where they overlap:

- **Builders** who want a Grok-style bot with their own personality, model and rules, without a closed platform.
- **Tinkerers** running local models (Ollama, llama.cpp, LM Studio) who want tools, memory and channels on top.
- **Teams** that need an internal agent whose prompts, logs and data never leave their machines.

The job: create an AI agent or social bot, shape its personality, test it, deploy it to the channels they choose, and stay in control of what it does.

## Product Purpose

TooluBot is an open-source studio for building your own AI agents and social bots: "Build your own agents. Own every line." Users run it on their own machine or their own keys and deploy to the channels they pick. Success: a first agent is chatting within a minute of first launch, and every agent stays a readable file the user owns.

## Positioning

Local-first and transparent where hosted bot platforms are closed:

- Agents are plain `*.agent.yaml` files in `~/TooluBot/agents/`, with no account required.
- Local models come first, and the model can be swapped without rewriting the agent.
- Every token, cost, latency figure and tool call is visible.
- Risky actions pause for human approval.

## Operating Context

- **Desktop-led, full parity.** Desktop is home base; web and mobile run the same full studio, adapted to screen size. No feature is desktop-only by design.
- **Areas:**
  - Agents: library and builder (Persona · Brain · Tools · Memory · Channels · Guardrails, with a live test chat beside it).
  - Flows: a visual canvas.
  - Tools & MCP.
  - Knowledge: indexed locally.
  - Channels: X, Discord, Telegram, Slack, Matrix, a webhook or a local HTTP API.
  - Runs: an audit log with replay.
  - Models: local models, plus remote keys stored in the OS keychain.
- **Journey:** Install → StarterPicker → Onboarding (pick a brain) → AppShell (say hi, shape the persona) → ToolsKnowledge → FlowEditor → FlowDebug → ChannelSetup → SafetyGoLive → Approvals → Runs (replay and tune, then back to the builder).
- **Approvals** arrive as a desktop notification and inline in the chat.

## Capabilities and Constraints

- **Stack:** Dioxus 0.7 Cargo workspace.
  - `packages/ui` holds all shared UI: tokens, primitives and screens.
  - `packages/web`, `packages/desktop` and `packages/mobile` are thin entry points that render `ui::Theme` and `ui::AppShell`.
  - `packages/api` holds fullstack server functions.
  - New UI goes in `packages/ui` so every platform gets it; platform crates add only platform wiring.
- **Built so far:** AppShell (the agent builder) on static demo data. Other areas and tabs are placeholders.
  - There is no model runtime yet. The test chat reports "not connected" rather than faking replies.
  - Nothing is saved to disk yet.
- **Approval rule:** anything that posts publicly, spends money or touches files outside the workspace requires approval by default.
- **Secrets** live in the OS keychain, never in the agent file.
- **Undecided:**
  - Licence. The design system says Apache-2.0, but nothing is confirmed and the repo has no LICENSE file. Do not claim a licence.
  - The model runtime and agent file persistence.

## Brand Commitments

- **Name:** TooluBot. The visual authority is the TooluBot design system: https://claude.ai/artifact/7eLRjAFNEkRzFeYSF9SdFv
  - The design system README still says "Tauri shell, React UI"; that is stale, since Dioxus replaced it.
- **Voice:** a sharp, friendly engineer. Short, specific, a little dry.
  - Never pretends the agent is a person, and never hides what a model did.
  - Sentence case everywhere, and buttons are verbs.
  - Errors say what happened and what to do next ("Couldn't reach Ollama at localhost:11434. Start it, then retry.").
- **Avoid:** blue-purple gradients, sparkle icons for "AI", robot mascots, glassmorphism, and emoji in UI chrome.
- **Mark:** `packages/ui/assets/toolubot-mark.svg` is a placeholder drawn for the design system; replace it when final artwork exists.

## Evidence on Hand

- The design system: tokens, 11 primitives, 11 screen comps and a placeholder logo.
- No users, testimonials, benchmarks, pricing or download numbers exist. Do not fabricate any.
- Demo agents (Scout, Dispatch, Pair, Lore) are illustrative only.

## Product Principles

1. **Yours, on disk.** Every agent is a file the user can read, version control, fork and share.
2. **Any brain.** Local first; any OpenAI-compatible endpoint, Anthropic, Mistral or Groq by key.
3. **See everything.** Tokens, cost, latency and every tool call are visible; nothing happens off-screen.
4. **Humans approve the risky bits.** Public posts, spending and file access outside the workspace pause for approval.
5. **Personality is a feature.** Tone, humour and reply length are first-class controls.
6. **One UI everywhere.** Build it once in `packages/ui`; every platform ships the same studio.

## Accessibility & Inclusion

- Text contrast is at least 4.5:1 on every surface in both the night and day themes.
- A colour signal always comes with a word.
- Every interactive element has a 2px focus ring.
- Ambient motion (the running-agent pulse) stops under reduced motion.
