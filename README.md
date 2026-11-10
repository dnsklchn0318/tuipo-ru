# tuipo-ru

Version 0.4.2. Local Russian and English spelling checks **while you type in
Codex CLI in Terminal.app**, with underlined mistakes and keyboard corrections.

## Start Codex with checking

From this source directory:

```sh
cargo build --locked --release
./target/release/tuipo -- codex
```

With a prepared archive, extract it and run `./tuipo -- codex` from the extracted
directory. Codex must already be available in PATH. To use another project, change
to that project's directory and invoke Tuipo by its absolute path followed by
`-- codex`. The wrapper preserves your working directory and forwards Codex arguments.

Tuipo starts Codex and checks input automatically. It does not add checking to a
Codex process that is already running; exit that process and start the command above.

## Correct a word

1. Type `превет world ` and wait briefly for the underline.
2. Move the cursor inside `превет` or immediately after its last letter.
3. Press **Ctrl+N** to open suggestions. Ctrl+N/P cycles forward/backward.
4. Press **Enter** to apply the selected correction, or **Escape** to dismiss.

Text changes only after confirmation. Submit the prompt with a separate Enter.
Tab keeps Codex completion by default. Russian and English are checked together;
no language switching is required. Terminal.app uses plain underline automatically.
The intended correction is not necessarily the first candidate.

## Local operation

The spelling engine and dictionaries run on your Mac. Tuipo has no cloud spelling
client or telemetry and does not save prompt text to its debug log. It checks the
selected terminal session; it does not scan projects or read .env files. The selected
application's own networking remains its own behavior.

The wrapper sees text typed in that session. It does not identify every secret field.
For a session without checking, simply launch `codex` directly. No global keyboard
listener, GUI permissions, background service or shell hook is installed.

```sh
./target/release/tuipo off     # stop checking; new launches bypass the wrapper
./target/release/tuipo on      # enable for the next launch; restart Codex
./target/release/tuipo status
```

## Settings and optional file checks

Read `$XDG_CONFIG_HOME/tuipo/config.toml`, otherwise `~/.config/tuipo/config.toml`.
Both languages and interactive corrections are on by default. Optional settings:

```toml
languages = ["ru_RU", "en_US"]
interactive = true
next_key = "ctrl+n"
previous_key = "ctrl+p"
underline = "highlight" # muted burgundy background; or auto/plain/fancy
pause_ms = 150
grammar = false
# dict_path = "/absolute/path/exceptions.txt"
```

Exceptions: one word per line, case-insensitive, `#` comments. The default exception
file is `dict.txt` beside config.toml. Custom Russian aff/dic paths must be supplied
together. Use `underline = "highlight"` for light text on a muted burgundy background.
`auto` retains the original terminal-dependent underline. The config TUI supports
all four styles. The original terminal settings remain supported.

`tuipo check -- draft.txt` is an optional file-only command, not the primary workflow.
It reads a regular UTF-8 file of at most 1 MiB without modifying it; exit codes are
0 no findings, 1 findings, 2 error.

Menus can pass keys through when the screen has insufficient free space, during
paste or with stale results. Complex graphemes and wide input characters disable
injected corrections conservatively. See [validation](docs/VALIDATION.md).

## Development and provenance

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
python3 scripts/check-local-policy.py
```

Based on [Tuipo by ARahim3](https://github.com/ARahim3/tuipo), MIT OR Apache-2.0.
Fork maintainer: `dnsklchn0318`. Russian uses Spellbook and LibreOffice Hunspell;
English uses Harper. ZIP/LRU updates remain in [vendor](vendor/README.md).
[Third-party notices](THIRD_PARTY_NOTICES.md) · [Dependency report](docs/DEPENDENCY_REACHABILITY.md)
· [Русская инструкция](docs/README.ru.md).
