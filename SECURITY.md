# Security model — 2026-11-10

Tuipo reads one explicitly selected UTF-8 file and configured dictionaries. It does
not observe Codex sessions, terminal output, other keystrokes or the clipboard.
The file is not edited. No shell hooks, startup services, PTYs, application launch
command or correction injection are supplied. The hidden `engine` command is an
internal bounded pipe protocol, not an application wrapper.

The checker runs in an owned subprocess with only HOME/XDG_CONFIG_HOME forwarded.
Input is at most 1 MiB; malformed UTF-8, special files and final-component symlinks
are rejected. Parent-directory symlinks are resolved normally: this is not a
filesystem sandbox. Configuration and exception/dictionary files are trusted local
inputs. Resources have not been exhaustively fuzzed and checking time is not bounded.

Selected text, findings and suggestions exist in RAM and are emitted intentionally
as command results. No content logging or persistent text cache is implemented.
TUIPO_DEBUG_LOG is unsupported and creates no file. Errors/panics suppress payloads;
child stderr is discarded. OS memory, shell history, terminal scrollback and output
redirected by the user are outside this control. Selecting a file containing a
secret explicitly gives that content to the checker; no secret detector is claimed.

The original raw-TUI design could not distinguish ordinary prose from secret
widgets. That risk was rejected for this implementation: the production terminal
proxy and automatic integration were removed, rather than relying on token patterns.

## Dependencies

See docs/DEPENDENCY_REACHABILITY.md, vendor/README.md and dependency-audit.json.
ZIP 9.5.0 and LRU 0.18.2 replace affected releases; no advisory ignores are used.
The unused Burn backend facade has been replaced by direct inference dependencies,
removing the inactive Candle/ZIP 7.x chain. bincode and paste are still unmaintained;
these warnings remain visible. Cargo build scripts execute as the developer user.
The local-only policy guard is a finite static check, not network isolation.

## Separate publication decisions

Source publication requires clean personal-data review, retained attribution and
licenses, accurate documentation and an explicit dependency report. An inability to
prove safety for confidential sessions is not a blanket prohibition on publishing
source. Binary redistribution additionally needs its license bundle and supported
platform validation. Installation/run and any future integration are separate
operations; none is performed by preparing the publication copy.

Private history backups are excluded from the publication copy. Original repository
history is preserved locally. Scanners are finite and do not certify absence of all
personal data or unknown secrets. No remote publication is part of this change.
