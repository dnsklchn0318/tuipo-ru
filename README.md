# tuipo-ru

Local Russian and English spelling support for selected terminal applications,
with underlined mistakes and keyboard-driven corrections. The executable is still
named **`tuipo`**. Russian uses Spellbook and a bundled LibreOffice Hunspell
dictionary; English uses Harper. No cloud spelling service is used.

**Publication and automatic integration are blocked by the security review.**
Known dependency vulnerabilities remain, and secret fields inside raw-mode TUIs
cannot be reliably identified. This checkout is for evaluation, not a claim of
safe use with confidential prompts. Read [SECURITY.md](SECURITY.md) and the
[publication audit](docs/PUBLICATION_AUDIT.md) before running it.

[Русская документация](docs/README.ru.md) · [Architecture and manual checks](docs/RUSSIAN.md)

> This project is based on Tuipo by ARahim3:
> https://github.com/ARahim3/tuipo
>
> The original project is licensed under MIT OR Apache-2.0. This fork extends Tuipo with Russian spell-checking and additional keyboard-driven correction features.

## Features

- Russian and US English enabled together, including ordinary Russian inflections.
- Existing Tuipo underline rendering; plain underline in Terminal.app.
- Background checking in a session-owned local subprocess.
- Suggestions at the cursor, explicit confirmation and stale-result rejection.
- Case-preserving Russian suggestions and common е/ё dictionary variants.
- Custom exception dictionary and heuristic filtering of URLs, paths, flags and code.
- Metadata-only diagnostics; no Tuipo input history or persistent text cache.

Examples: `работающие люди read English words` is checked without switching
languages. In `превет world `, the Russian typo is underlined after a pause.
Open the suggestions and select `привет`; the first candidate is not necessarily
the intended word. Spellchecking cannot identify every real-word mistake.

## Build on macOS

Rust and Cargo are required. This checkout has been tested on macOS 27 ARM64 with
Rust 1.99.1. Build from the existing source directory:

```sh
cargo build --locked --release
./target/release/tuipo -- codex
```

Run that command in Terminal.app. It wraps only this Codex invocation. Ordinary
commands in other terminal sessions are not intercepted. There is no macOS global
keyboard listener, Accessibility/Input Monitoring integration, LaunchAgent or daemon.

After the security gate is resolved, a local Cargo installation would be:

```sh
cargo install --path . --locked
```

That installs the `tuipo` executable into Cargo's bin directory. It does not install
shell integration. No public fork or release has been published during this review.
The package's original repository metadata remains upstream until publication is
approved; do not mistake the upstream releases for builds of this fork.

## Correct a word

1. Finish a word with a separator and wait briefly for its underline.
2. Move the cursor inside the typo or immediately after its last letter.
3. Press Ctrl+N, choose a candidate, and press Enter.

| Key | Action |
| --- | --- |
| Ctrl+N | Open suggestions / select next, cyclically |
| Ctrl+P | Open at the last candidate / select previous, cyclically |
| Enter | Apply the selected candidate |
| Escape | Close without editing |

Opening and cycling never change text. Enter applies only while a menu is engaged;
submit the prompt separately. A shortcut passes to the app if the result is stale,
there are no useful suggestions, or the menu cannot be positioned safely. The first
shortcut during engine startup can therefore reach Codex. Tab retains completion
unless the existing `tab_fix` or legacy `picker` option is explicitly enabled.

## On, off and status

From this checkout:

```sh
./target/release/tuipo on
./target/release/tuipo status
./target/release/tuipo off
```

`on` permits checking on **subsequent explicit app launches**. It starts no service,
installs no hook and does not bypass the automatic-integration security gate.
`off` makes new invocations execute the selected app directly. A running wrapper
switches to byte forwarding without spelling buffers, screen reconstruction or
shortcut capture. Once disabled, that wrapper stays forwarding-only until the app
exits. Use `on` and restart Codex to resume checking with fresh screen state.
`status` reports the configured mode and the blocked automatic-integration gate.
These commands do not inspect other terminal sessions.

## Configuration

Malformed or unreadable settings bypass checking with a content-free warning;
an absent file uses defaults. Settings are read at startup from `$XDG_CONFIG_HOME/tuipo/config.toml`, otherwise
`~/.config/tuipo/config.toml`. Existing Tuipo options remain supported:

```toml
languages = ["ru_RU", "en_US"]
interactive = true
highlight_only = false
next_key = "ctrl+n"
previous_key = "ctrl+p"
pause_ms = 150
underline = "auto" # plain in Terminal.app; or force "plain"
grammar = false    # optional narrow English rules; no Russian grammar
# Legacy features, off by default:
tab_fix = false
picker = false
status_row = false
hover_ms = 250
# Optional UTF-8 dictionary pair; supply both together:
# russian_aff_path = "/absolute/path/ru_RU.aff"
# russian_dic_path = "/absolute/path/ru_RU.dic"
# Exceptions: one word per line, # comments, case-insensitive:
# dict_path = "~/my-tuipo-dict.txt"
```

Set `interactive = false` to disable the new keys, or `highlight_only = true` to
also disable legacy interactive correction. Keys accept `ctrl+a` through `ctrl+z`;
avoid Enter, Tab, interrupt and other important application bindings. The config
TUI preserves the new settings, but they must be edited in TOML.

Turning off painting or choosing highlight-only does **not** stop text processing.
For a session with no Tuipo processing from the start:

```sh
TUIPO_PRIVATE=1 ./target/release/tuipo -- codex
```

This executes Codex directly without a Tuipo PTY, buffer, engine or debug file.
SSH-marked and nested wrapper launches also bypass processing. This does not change
Codex's own network requests, history or retention settings.

## Privacy and limitations

Spelling stays on the machine; telemetry and transmission of text by Tuipo are
prohibited. Optional `TUIPO_DEBUG_LOG` contains technical metadata only. Tuipo does
not save prompts, suggestions or terminal transcripts. Older content logs and the
selected application's own files are not erased or controlled by this fork.

Canonical no-echo password entry bypasses checking. Raw-mode password/token fields
and secrets typed as ordinary prose cannot be reliably distinguished. Do not use
automatic wrapping for those scenarios. No shell function is installed or offered
for activation while this limitation and dependency findings remain unresolved.
See [the integration design and gate](docs/INTEGRATION.md).

Other known limits include custom editor bindings/history, crowded Codex footers,
redraw races, wrapped menus and complex Unicode graphemes. Combining sequences,
emoji and wide characters inside input conservatively disable correction and
input overlays. Earlier pasted lines are not corrected. Hostile app escape output
is forwarded unchanged. Visual behavior in native Terminal.app and an interactive
Codex session has not been verified by this audit.

## Tests and dependency audit

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
python3 scripts/check-local-policy.py
python3 scripts/audit-repository.py
```

The last script reports paths/rule names without printing matching secret values;
it is a heuristic, not proof of no secrets. It intentionally fails on unresolved
historical metadata findings. Cargo.lock is committed; use `--locked` for reproducible
resolution. See [validation](docs/VALIDATION.md) and [RustSec results](docs/dependency-audit.json).
The dependency audit currently fails; findings are not hidden with ignore rules.
The inherited release workflow and upstream Homebrew publication configuration
have been removed. There is no automated publishing pipeline in this fork.

## Disable and uninstall

Run `tuipo off` to stop processing in running wrappers and bypass subsequent launches.
For immediate termination of the proxy, exit the selected app. Launch Codex directly
with `codex` afterward. No shell/LaunchAgent cleanup is needed for this review:
no integration has been installed, and `tuipo init` refuses installation.

If you installed this checkout with Cargo, use `cargo uninstall tuipo`. If you only
ran `target/release/tuipo`, there is no installed program to remove. You may manually
remove Tuipo's config directory and any debug file you explicitly selected after
reviewing them. Earlier versions' manually installed hooks require removing their
exact definitions from your own shell configuration; this fork does not rewrite
user files or delete unrelated functions/settings.

## Libraries, dictionaries and licensing

Tuipo's source remains **MIT OR Apache-2.0**; [LICENSE-MIT](LICENSE-MIT) and
[LICENSE-APACHE](LICENSE-APACHE) and original credits are retained. Harper is
Apache-2.0; Spellbook is MPL-2.0. The Russian dictionary has a separate permissive
license from Alexander I. Lebedev, with a documented modification by Laszlo Nemeth.
Redistributions must include its notice. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)
and [dictionary provenance](assets/ru_RU/SOURCE.md).

Thanks to ARahim3 for the original PTY, rendering, buffering, picker and correction
architecture, and to the Harper, Spellbook and LibreOffice dictionary contributors.
This fork does not claim authorship of the original Tuipo implementation.
