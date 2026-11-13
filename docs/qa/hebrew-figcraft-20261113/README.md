# FigCraft Hebrew native RTL visual QA — 2026-11-13

**Source PR**: https://github.com/storytold/figcraft/pull/2239
**Exact captured source commit**: `eba0c20e40591a1c2fae94cc9898b2cfed9fc768` (the source PR's head at capture time).
**Capture host**: MacMiniOffice2, Apple Silicon macOS, isolated `FIGCRAFT_CONFIG_DIR` and screenshot-only automation root. Built with `cargo build -p figcraft --locked` (exit 0) using the pinned PR worktree; screenshot resolution 1440 x 900. UI changed through the native loopback control protocol, with no edits to the user's own app profile/files. These are actual app pixels, **not a mockup or older-head capture**.

| Screenshot | Surface / test |
| --- | --- |
| [01-home-he.png](screenshots/01-home-he.png) | Home view with Hebrew locale, navigation, starter cards |
| [02-document-inspector-he.png](screenshots/02-document-inspector-he.png) | Open Hebrew-named test document + selected iPhone frame; design inspector left, document/layers right |
| [03-inspect-he.png](screenshots/03-inspect-he.png) | Inspect/dev panel state, selected frame, Hebrew/mixed English fallbacks |
| [04-command-palette-he.png](screenshots/04-command-palette-he.png) | Command palette with Hebrew action labels |
| [05-file-menu-he.png](screenshots/05-file-menu-he.png) | File menu and action submenu in Hebrew |
| [07-version-history-he.png](screenshots/07-version-history-he.png) | RTL version history panel with a scratch-only Hebrew snapshot |
| [09-export-section-scrolled-he.png](screenshots/09-export-section-scrolled-he.png) | Expanded/scrolled export section, PNG options and preview control |

**Review notes:** The visible main document, menus, palette, version timeline and export section show readable Hebrew glyphs without obvious missing-glyph boxes at this resolution. The panels and tab bar show the expected mirror geometry. A number of English strings remain (for example Inspect developer-only controls, Branches and starter-card explanatory text), because this is a *partial draft Hebrew catalog*. The screenshots are evidence for **these specific captured states only**. Full keyboard/screen reader access, all responsive sizes, clipped text across other dialogs, all Linux/Windows native configurations and comprehensive native-speaker review remain outside this screenshot pass.

**Complementary executable tests on this exact source head:**
- `cargo check -p figcraft-ui-egui --tests --locked`: PASS.
- `cargo test -p figcraft-ui-egui i18n --locked`: 37 passed, 0 failed.
- Additional targeted native tests (same compiled test binary): 14 Hebrew, 7 mirror, 1 history (one overlap between filter groups), all PASS.

The official upstream CI and SonarCloud actions were `action_required` at capture time, not claimed passing. This evidence branch deliberately does not modify the source PR's branch or code. Screenshot SHA256 checksums are in [SHA256SUMS](SHA256SUMS).
