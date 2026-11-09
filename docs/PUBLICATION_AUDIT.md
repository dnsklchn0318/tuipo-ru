# Publication gate review — 2026-11-10

**Result: BLOCKED. No GitHub publication or automatic shell activation.**
The repository stays on the working feature branch. No remotes, upstream repository,
`main`, user dotfiles, system settings or startup services are changed by this review.
GitHub CLI is installed and `gh auth status` succeeded, but that is not a security
approval. No fork creation, merge, push or Actions execution was requested remotely.
The existing origin still points upstream; do not push this working branch there.

## Gate findings

| Gate | Finding / action |
| --- | --- |
| Known high/critical dependency vulnerabilities | FAIL. Eight RustSec records remain for zip 7.2.0/8.6.0, plus lru unsoundness and two maintenance warnings. Rechecked against the current fetched database, no ignores. Current Harper 3.0.0 still declares zip 8.6.0; a simple engine upgrade does not remove this blocker. Replacing major ZIP APIs or carrying security forks requires a separately reviewed/tested upstream adaptation, not an unverified version override. |
| Secret exclusion in automatic Codex wrapping | FAIL. A PTY sees raw-mode secret widgets and ordinary prose in the same stream. Termios/echo heuristics do not establish a nonsecret boundary. No automatic function or installer has been created. |
| Repository privacy/history | BLOCKED for review. Credential signatures found no match, but two older documentation blobs contain a local home path and a historical blob is `.DS_Store`. Current docs are scrubbed; history is preserved. Binary screenshots/GIF and commit identities have not been certified free of personal information. No automatic history rewrite. |
| Tuipo text exfiltration / telemetry | No runtime client or call found in reviewed application code; policy guard passes on the active normal/build graph across all targets. This is not an OS network sandbox or proof against malicious dependencies. The selected app retains its own network behavior. |
| Prompt logging / persistent caches | Content-free API and canary tests pass. No Tuipo input-history/persistent text cache writer found. Debug files are private and metadata-only. Old logs, app history, OS swap and terminal scrollback are outside this control. |
| Off mode | Fixed: no spelling buffer or screen reconstruction after the switch is observed. New launches directly execute the selected app. Existing proxies remain forwarding-only until exit; on requires restarting the app. |
| Failure handling | Persistent engine failure now disables checking; no production in-process fallback. Known owned input/lint snapshots are invalidated. |
| Scope / SSH / children | Only the selected PTY is observed. SSH-marked and nested launches bypass processing. No global keyboard API or daemon. Engine env contains only config-path variables, never the full API-token environment. |
| Unicode correction | Existing positive tests and conservative grapheme/control/span/paste rejection pass. Interactive Codex/Terminal.app GUI behavior is not verified. |
| License/provenance | Original licenses/credits unchanged. Spellbook MPL-2.0 notice and separate Russian dictionary notice retained. Active dependency SPDX inventory has no missing license declarations; it is not a complete legal review of every bundled upstream model. |

## Scope and source review

Reviewed `main`, PTY forwarding/capture, input/buffer state, engine framing/spawning,
worker lifecycle and stale-result handling, debug writes/panic handling, screen
reconstruction, picker/fix validation, configuration writes and init/on/off commands.
The selected app necessarily receives terminal bytes and may itself send/store them.
Tuipo cannot make Codex or a remote SSH application confidential.

`off` previously suppressed lints while continuing to rebuild the input/screen.
That behavior has been removed. The mode is checked before each input batch and
render event. In-flight batches, queued byte transport and OS memory are not an
atomic or securely erased boundary; use direct/private launch before secrets.
The proxy cannot turn an already-running PTY app into a direct exec, so it keeps
only transport until app exit. It observes no other terminal or keyboard source.

On/off state uses create-new private files, rejects following an existing/dangling
switch symlink, and treats missing/unreadable mode configuration conservatively.
The debug API does not accept text payloads. Configuration saves use new private
temporary files instead of truncating a predictable `.tmp` target. Malformed or
unavailable config bypasses processing, suppressing TOML snippets. Raw child launch
errors that could include PATH/arguments are suppressed. Menus refuse terminal
controls in dictionary suggestions. Tests cover these paths. No new shell-history collection,
credential reads, project scans or auto-installers were introduced at runtime.
The repository scanner is a developer tool, not code called by the spellchecker.

Build review found an inherited release workflow with write access, moving Action
tags, shell installers fetched by curl, and publication to the original author's
Homebrew tap. That workflow and cargo-dist publication configuration were removed.
No replacement release pipeline is enabled. Cargo's third-party build scripts still
execute under the developer's account; the lockfile is not a sandbox. The guard
checks active normal/build dependencies, not a complete line-by-line audit of every
transitive crate or native toolchain component.

## Dependency and dictionary evidence

cargo-audit 0.22.2: RustSec database commit
`d9cb059f3217db87aeccd190b81eaffe8cc29cb9`, 1,430 advisories. Recheck exits 1.
See [the full report](dependency-audit.json) and [SECURITY.md](../SECURITY.md) for IDs,
versions and upstream constraints. Earlier compatible updates remain locked; no
blanket-ignore policy was added. No claim is made that the remaining ZIP flaws are
harmless just because Tuipo does not expose archive extraction.

Harper 3.0.0's downloaded Cargo manifest was inspected as a candidate replacement:
it still has `zip = "8.6.0"`. A major Harper migration therefore was not applied as
a supposed complete fix. API churn and a changed English lint baseline would also
need regression review. Publication remains refused rather than weakening the gate.

The bundled Russian aff/dic are byte-for-byte checked against LibreOffice commit
`c0865a85eb67dc377f2cf8e8fe2c4fa9e53f0dcb`. Local SHA-256 values:

- aff: `38ce7d4af78e211e9bafe4bf7e3d6a2c420591136cb738ec6648f8fdf6524cd7`
- dic: `f6047416a0204adbecf3a451b874ec8a97ee37e2cbc714466ef04d8dbcc0d6fc`

Spellbook uses local Hunspell data; no executable dictionary script is installed.
Custom dictionaries/configuration must still be trusted, and parser/resource bugs
remain within the threat model. All 288 active normal/build package-version entries
have license declarations in [the SPDX inventory](dependency-licenses.json), covering
275 distinct crate names across targets. The LGPL alternative in one OR expression
can be satisfied using its permissive alternative. File-level MPL and Unicode
notices must be carried in any later binary redistribution.

## Repository inspection

The offline scanner reviews current tracked plus nonignored candidate files and all
blobs reachable from local refs. It emits only object IDs, paths and rule names.
Its finite rules cover common private keys/provider credentials, credential URLs,
forbidden local paths and local home-path metadata; they do not prove absence of
arbitrary secrets or personal data. Results: [repository-audit.json](repository-audit.json).

There were no currently tracked agent instructions, environment files, private keys
or debug logs requiring untracking. No local copies were deleted. `.gitignore`
now excludes nested AGENTS/AI-state directories, environment/local-secret files,
keys, SSH/AWS state, logs/history/recordings, temporary files, target and macOS/editor
metadata. Tests of ignore patterns preserve Cargo.lock, public source/tests,
dictionaries, docs and licenses. Ignore rules cannot erase historical blobs.

Publication requires an explicit decision about historical metadata and review of
binary demo assets/commit identities. If a real credential is discovered later,
rotate it before considering a separately authorized history-repair procedure.
No force push or history rewrite is performed here.

## Validation

On macOS 27.2 ARM64 with Rust 1.99.1:

- `cargo fmt --check`: passed.
- `cargo test`: 317 unit + 27 integration tests passed; one existing diagnostic probe ignored.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo build --release`: passed.
- Local-only dependency/source policy guard: passed.
- Repository heuristic scan: no current signature findings; historical metadata findings, exit 1.
- RustSec audit: unresolved findings, exit 1.

Tests include on/off/status persistence, idempotent switches, no text/spelling events
after off, fresh-launch bypass, SSH/nested bypass, failure shutdown, private switch
symlinks, raw UTF-8-state disposal, logging canaries, no-echo PTY password input,
Unicode replacements, paste, positioning and existing process restart/exit checks.
Cargo package file-list review is local only; no crate/package upload is performed.

Not performed: native Terminal.app GUI interaction, interactive authenticated Codex,
real SSH authentication, relaunching the GUI terminal, global macOS event interception
(none implemented), runtime network packet tracing, exhaustive fuzzing, independent
security review, or approval of a complete binary-release license bundle.
No claim is made that those checks passed. Earlier Zsh PTY smoke evidence is recorded
in VALIDATION.md and is not a substitute for these scenarios.

## What can happen next

Resolve the ZIP/lru dependency baseline through reviewed compatible upstream fixes
or tested backports; review historical metadata/assets; obtain a verified safe prose
boundary before reconsidering automatic Codex wrapping. Then repeat the full gate.
Only after success may the existing authenticated GitHub CLI be used to inspect/create
the user's official fork, set origin/upstream, integrate main without data loss and
push only to that fork. Only after publication and an exact user-file diff may
separate confirmation be requested for installation. None of those actions is
performed while this report says BLOCKED.
