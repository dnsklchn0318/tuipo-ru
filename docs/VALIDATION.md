# Validation — explicit file checker, 2026-11-10

Environment: macOS ARM64, Rust 1.99.1. This report supersedes terminal-overlay
validation from earlier commits; old PTY tests were removed with the proxy.

- `cargo test --locked --offline --quiet`: 51 unit tests passed, one inherited
  Harper diagnostic ignored; 5 file-interface tests and 2 dependency regressions passed.
- `cargo clippy --locked --offline --all-targets -- -D warnings`: passed.
- `cargo fmt --check`, `git diff --check`: passed.
- `cargo build --locked --offline --release`: passed.
- Local-only guard: passed, 254 active normal/build crate names across all targets.
- License inventory: 269 package/version entries, none missing a declared license.
- cargo-audit 0.22.2, fetched RustSec DB
  d9cb059f3217db87aeccd190b81eaffe8cc29cb9 (1,430 advisories): zero vulnerabilities,
  no unsound warnings; bincode/paste maintenance warnings remain. A repeat outside
  the sandbox completed including access to the crates.io index. `--deny warnings`
  deliberately fails; no ignore configuration is present.

File tests cover bilingual positions, empty/correct/invalid/oversize input,
nonmodification, symlink/FIFO refusal, removed app-launch syntax, exception files,
configuration diagnostic canaries, and absence of a requested debug file. Unit tests
cover control/bidi escaping, bounded UTF-8 reading and the retained RU/EN behavior.
Dependency regressions exercise ZIP 9 Weirpack round-trip with a Cyrillic name,
malformed archive rejection, and LRU pop with a panicking key destructor followed
by further cache insertion/eviction. The fixtures do not constitute exhaustive
upstream tests or an exhaustive review of ZIP/LRU APIs.

The production dependency graph has no terminal/PTY or clipboard library; main only
spawns its own engine with a cleared environment. Engine stdin is its internal
pipe, not a stream captured from Codex. No native GUI/Codex verification is claimed
or required for this file interface. Additional platforms and binary license bundles
have not been validated. No installation or remote publication was performed.

## Publication copy

The repository scanner's committed publication report records the implementation
commit reviewed before the report-only commit. It scans current files plus every
blob and commit identity reachable from local refs. The final report-only commit
is checked again externally and contains no application changes. This avoids a
self-referential claim that a report's own commit hash can occur inside the report.
The original working repository intentionally retains its unsanitized history;
its scan still fails. Only the separate publication copy has sanitized history.
