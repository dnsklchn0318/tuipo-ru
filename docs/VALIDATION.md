# Validation of the RU/EN fork

Environment: macOS 27.2 (26B123), Darwin ARM64. Working branch:
`feat/russian-spellcheck`. Tested in the existing local checkout at
`/home/USER/Desktop/Work/Personal/tuipo-ru`.

| Check | Result |
| --- | --- |
| `cargo fmt --check` | Passed |
| `cargo test` | 308 unit tests + 19 integration tests passed; one original diagnostic Harper probe ignored |
| `cargo clippy --all-targets -- -D warnings` | Passed |
| `cargo build --release` | Passed |
| `git diff --check` | Passed |

The initial source tree required rustfmt changes outside the feature modules;
these are included to make the repository-wide formatting check pass.

New tests exercise Russian inflections, common е/ё variants, mixed RU/EN prose,
case-preserving suggestions, user exceptions, ignored technical text, Unicode
span validation, replacements of different lengths at different cursor positions,
conservative rejection of ambiguous graphemes, configurable/disabled keys,
Ctrl+N/P cycling, Enter confirmation, Escape dismissal, edits invalidating a menu,
unsafe-menu and paste passthrough, Cyrillic wrapping and ordinary wide prompt cells.
The real engine protocol test now checks both English and Russian lints. All
original tests are retained, including multiline paste, PTY forwarding, underline
rendering, Tab fixes, resize/cursor models and engine lifecycle checks.

Additional smoke test used an actual PTY at 120×30 with the release binary wrapping
`zsh -f`, an isolated temporary config directory, `TERM=xterm-256color` and
`TERM_PROGRAM=Apple_Terminal`. No user dotfiles were changed. Typed
`print -r -- "превет"`, moved the cursor before the closing quote, opened the
menu with Ctrl+N, chose its last candidate `привет` with Ctrl+P, confirmed with
Enter and submitted with a second Enter. Captured Zsh output contained `привет`;
Zsh exited successfully. This validates ordinary Cyrillic ZLE editing and confirms
that opening the menu and applying a replacement are separate actions. It does
not establish compatibility with other Zsh bindings or plugins. Spellbook's first
candidate for this typo was `ревет`; selecting the intended candidate matters.

Codex CLI is present locally, but an interactive Codex session and native
Terminal.app GUI rendering were not tested. Footer redraws, scrolling, native
resize and visual cleanup remain manual checks in [RUSSIAN.md](RUSSIAN.md).
Do not interpret simulated screen/model tests as native GUI verification.

Known limits: no Russian grammar, dictionary-dependent proper names and е/ё
context; heuristic CLI/code filtering; injected edits depend on the host's cursor
and Backspace semantics; complex graphemes/wide characters in input conservatively
disable overlays and correction; corrections on earlier pasted lines are refused;
menus require an unoccupied, unambiguous position and may pass keys through in
crowded/narrow screens. See the implementation document for details.
