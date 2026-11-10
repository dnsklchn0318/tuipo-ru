# Version 0.4.0 validation — 2026-11-10

Environment: macOS ARM64, Rust 1.99.1, installed Codex CLI 0.167.0.
Terminal checking is the primary interface; file checking is optional.

- `cargo test --locked --offline --quiet`: 319 unit tests, 2 dependency regressions,
  5 file-interface tests and 27 terminal integration tests pass; one inherited
  Harper diagnostic is ignored (353 passing tests in total).
- `cargo clippy --locked --offline --all-targets -- -D warnings`: passed.
- `cargo fmt --check`, `git diff --check`: passed.
- Release build with remapped build paths: passed.
- Local-only policy guard: 276 active normal/build crate names across all targets.
- ZIP 9.5.0 and LRU 0.18.2 remain fixed; bincode/paste maintenance warnings are
  disclosed in dependency-audit.json without ignores.

A real PTY at 120x30 wrapped `zsh -f` with isolated temporary configuration and
TERM_PROGRAM=Apple_Terminal. Typed `print -r -- "превет"`, moved into the typo,
opened suggestions with Ctrl+N, selected `привет` with Ctrl+P, confirmed and submitted.
Zsh received and printed the corrected text. Menu opening and replacement were
separate actions. This was run against the prepared release binary.

A second real PTY at 140x40 launched the installed Codex CLI through Tuipo. Typed
only a synthetic Russian draft, moved to the typo and pressed Ctrl+N. The Russian
suggestion menu rendered. No prompt was submitted to Codex. This is an actual CLI
integration smoke check; native Terminal.app GUI redraw/resize was not observed.

Tests retain passthrough, underline ANSI, Unicode spans, stale-result rejection,
paste, engine lifecycle, on/off behavior, no-echo input and metadata-only log canaries.
The optional file checks and ZIP/LRU regressions remain tested. No global install,
shell-profile edit or Codex-setting change is part of this implementation.
