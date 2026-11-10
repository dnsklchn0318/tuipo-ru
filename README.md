# tuipo-ru

Version 0.3.0 — explicit file checking.

Local Russian and English spelling checks for an explicitly selected UTF-8 file.
The executable is `tuipo`. Russian uses Spellbook and the bundled LibreOffice
Hunspell dictionary; English uses Harper. Text is not sent to a spelling service.

```sh
cargo build --locked --release
./target/release/tuipo check -- draft.txt
```

Results are `line:column: word -> suggestions`, with one-based Unicode scalar
columns. Suggestions are informational: the file is never modified. Exit status:
0 means no findings, 1 means findings, 2 means a read/configuration/checking error.
Files must be regular UTF-8 files of at most 1 MiB; final-component symlinks and
special files are refused. Terminal control and bidi formatting characters are
escaped in results. Shell redirection can deliberately save results containing text.

Tuipo does not wrap Codex, start applications, intercept terminal input, access the
clipboard or install shell integration. Draft text in your own editor, check its
file, then transfer it yourself. Files explicitly selected for checking can contain
secrets: Tuipo will read those too. The selected text exists in process memory.

## Configuration

Read-only settings: `$XDG_CONFIG_HOME/tuipo/config.toml`, otherwise
`~/.config/tuipo/config.toml`. Missing settings use defaults; invalid settings fail
with status 2 and a content-free diagnostic. Old terminal settings must be removed
manually; Tuipo never rewrites your configuration.

```toml
languages = ["ru_RU", "en_US"]
grammar = false
# dict_path = "/absolute/path/exceptions.txt"
# russian_aff_path = "/absolute/path/ru_RU.aff"
# russian_dic_path = "/absolute/path/ru_RU.dic"
```

Both Russian dictionary paths must be supplied together. Exceptions contain one
word per line, case-insensitive, with `#` comments. The default user exception file
is the config directory's `dict.txt`. These explicitly configured resources must be
trusted. The checker subprocess receives only HOME/XDG_CONFIG_HOME, not the full
parent environment. It is a memory-lifetime boundary, not an OS sandbox.

## Status and checks

This is an experimental source checkout. Source publication readiness, dependency
security, binary redistribution and installation/run decisions are separate.
See [publication review](docs/PUBLICATION_AUDIT.md), [security](SECURITY.md),
[dependency reachability](docs/DEPENDENCY_REACHABILITY.md) and
[validation](docs/VALIDATION.md). No installer or publishing workflow is enabled.

```sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
python3 scripts/check-local-policy.py
python3 scripts/audit-repository.py
```

Known ZIP/LRU advisories are addressed by documented local Harper adaptations in
[vendor](vendor/README.md). Maintenance warnings for bincode/paste remain visible
in the committed audit. Neither successful scans nor local execution prove absence
of unknown vulnerabilities. Spelling is dictionary-based, with heuristic code
filtering and no Russian grammar or reliable detection of real-word errors.

Based on [Tuipo by ARahim3](https://github.com/ARahim3/tuipo), MIT OR Apache-2.0.
The original terminal implementation remains in Git history; this version checks
selected files. Original credits and licenses are retained. Fork maintainer:
`dnsklchn0318`. See [third-party notices](THIRD_PARTY_NOTICES.md).

[Русская документация](docs/README.ru.md)
