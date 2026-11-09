# Security policy and threat model

This is a local terminal spellchecker, not a security boundary around the selected
application. Security controls below are supported by source review and regression
tests; they do not establish that this project is completely secure.

## Data flow and trust boundaries

Launch only the application you choose, for example:

```sh
./target/release/tuipo -- codex
```

Tuipo proxies stdin to that application's PTY. It reconstructs an input buffer in
memory; a worker sends text over anonymous local pipes to the same executable's
`__engine` process. Harper and Spellbook check that text locally using embedded
models/dictionaries or an explicitly configured local Hunspell pair. Results return
over pipes. Terminal output is forwarded to stdout and modeled in an in-memory grid
for positioning. There is no socket server, network spellchecker or automatic
model/dictionary download at runtime.

Network transmission of input, telemetry, analytics, remote crash reporting and
network-based spelling services are prohibited in Tuipo. No such client or runtime
network call was found in this audit. `scripts/check-local-policy.py` rejects known
HTTP/telemetry packages in the active normal/build graph and common network entry
points in `src`. This finite guard is not proof against arbitrary or obfuscated
network code; Tuipo is **not** an OS network sandbox. Future dependencies/features
must undergo the same review. Build-time Cargo downloads and developer RustSec
updates transfer public dependency metadata, not terminal contents.

**The selected application remains independent.** Codex can send prompts to its own
services and save its own history, transcripts and logs. Tuipo does not disable or
control those actions. Wrapping a shell can likewise leave that shell's history
behavior intact. To change Codex privacy/retention, configure Codex separately.

The engine is launched directly using the current executable, without a shell. It
gets only `HOME` and `XDG_CONFIG_HOME` for configuration, rather than inheriting API
tokens and the application's entire environment. Framed IPC has length/allocation
checks, a 64 MiB frame cap and a 256 KiB lint-input cap. Stale results are rejected
by exact-text/epoch checks. Persistent engine failures disable checking for the session and retain only byte
forwarding. There is no production in-process fallback; the old
`TUIPO_ENGINE=inprocess` override is no longer honored. Process separation is fault
containment, not an OS sandbox.

The selected application necessarily receives its arguments, normal environment,
working directory and stdin. An attacker controlling that executable, PATH, local
configuration, dictionaries or Tuipo's binary/dependencies is outside the guarantees
of these controls. Do not run untrusted apps/dictionaries under the assumption that
Tuipo will constrain them.

## Logging and persistence

Content logging has been removed, not merely disabled by default. There is no
switch to restore it. The debug API accepts event labels and numeric metadata;
input/output text, lint words, suggestions, correction contents, child diagnostics,
API tokens, passwords, environment values and command arguments are never written
to the debug log. Arbitrary formatted engine-error messages are not logged. Panic
hooks suppress panic payloads and backtraces, including in `__engine`.

`TUIPO_DEBUG_LOG=/path` is opt-in and records elapsed times, event types, issue and
print counts, engine PIDs/lifecycle, terminal dimensions/cursor and overlay counts.
Even metadata reveals timing/activity; leave this variable unset when unnecessary.
New log files are created with mode 0600. Symlinks, hard-linked files, nonregular
files and existing files accessible to group/others are rejected without writing.
Failure to open a suitable log simply disables logging. Protect the parent directory
as well; an attacker running as the same user can interfere with files/processes.

Tuipo does not write prompt history, original messages, terminal transcripts,
spelling inputs or suggestions to disk. Ordinary launch reads only configuration,
the custom exception dictionary, optional configured dictionary files, the on/off
switch and OS terminal/process facilities. It does not scan projects, home folders,
credentials, user identity or unrelated files. Config/path environment variables,
terminal-style flags, executable location and cwd are used only for their operational
purpose. Explicit `tuipo config`, `off` and `on` can write settings/switch state;
these contain no input history. Config saves create a new mode-0600 temporary file
with create-new semantics before atomic rename; existing files/symlinks cannot be
truncated through that temporary path. Invalid/unreadable configuration disables
input processing and launches the selected app directly (a missing config uses
defaults). Config errors suppress TOML contents, and launch errors suppress raw
child diagnostics that may include PATH/arguments. `off` is checked before each input batch and render
event (including periodic ticks). New launches bypass the PTY. Existing wrappers
clear their inferred input/screen/lint state and stop the engine, then remain byte
forwarders until exit. `on` affects subsequent explicit app launches; it never
reconstructs an old disabled session. Snapshots are invalidated when switching off.
Already-read batches and switch transitions cannot form an atomic privacy boundary;
use direct launch before entering sensitive data. An explicitly configured exception file is read,
not populated automatically from prompts.

Shell-hook installation (`tuipo init`) is disabled, including `--dry-run` and
`--force`. There are no autostart registrations or background services. The engine
is a session-owned child, stopped when idle/shutdown occurs; it is not a daemon.
Existing hooks or content logs installed by earlier builds are not scanned, edited
or erased automatically. Review/remove them yourself if applicable.

## Hidden input and secrets

Before processing each stdin batch, Tuipo queries the child's PTY termios. If
`ICANON` is set and `ECHO` is clear (standard canonical password input), it discards
the inferred input/selection, invalidates pending lint results and forwards the
batch directly without buffering, spellchecking or shortcut capture. Failure to
query the mode uses the same forwarding-only behavior. On resumption, reconstruction
starts from an empty input state. Tests exercise a real `stty -echo` / `read` session.

**Raw TUI password widgets are not reliably detectable.** Most TUIs already disable
kernel echo for normal editing, so no-echo raw mode cannot distinguish a password
from prose. Masked widgets, app-specific secret modes, termios transitions racing
an input batch, pasted secrets before a mode change, or secrets entered as ordinary
text may still enter Tuipo's memory and local engine. Grid anchoring is a positioning
check, not a confidentiality check. We do not promise to protect such inputs.

For sensitive sessions, bypass all Tuipo processing from the start:

```sh
TUIPO_PRIVATE=1 ./target/release/tuipo -- codex
```

This execs the selected application directly, without a Tuipo PTY, checker, buffer,
overlay or debug file. It does not make that application private. Merely disabling
painting or enabling highlight-only mode does **not** disable input processing.

Discarding buffers is logical disposal, not secure memory erasure. Allocators,
Harper caches, OS swap, crash/core dumps, terminal scrollback, debuggers, screen
recorders and a compromised account/OS can retain or expose content. No claim is
made to defend against root, malware, same-user process inspection or forensic
memory recovery. Older logs and the wrapped application's own files are outside
Tuipo's control.

## Editing and terminal-output risks

Corrections require explicit confirmation and a fresh matching word/span. Unsafe
Unicode edits, injected controls/newlines, active paste and ambiguous menu positions
are refused. Menu rendering also rejects control/non-unit-width suggestion text
rather than emitting terminal commands from dictionary data. The selected app still owns editing semantics; custom bindings/history,
mouse editing and redraws can desynchronize the inferred buffer. Terminal escape
output from the app is forwarded unchanged: hostile OSC/clipboard/window sequences
and malicious terminal output are not sanitized by this wrapper. A noisy child,
huge input or unbounded event queues can cause resource exhaustion. Do not treat
Tuipo as a terminal emulator sandbox or an input-integrity guarantee.

## Dependency audit (rechecked 2026-11-10)

`Cargo.lock` is committed. Use `cargo build --locked --release`, `cargo test --locked`
and `cargo install --path . --locked` to preserve it. Cargo.lock pins the tested
resolution; it does not ensure dependency code is benign.

Audited with cargo-audit 0.22.2 and the RustSec database at commit
`d9cb059f3217db87aeccd190b81eaffe8cc29cb9` (1,430 advisories; database updated
2026-11-09). The original lockfile had **33 vulnerability records** plus warnings.
Compatible dependency updates and disabling Harper's unused optional thesaurus
removed 25 of those records and multiple unsoundness warnings. The disabled thesaurus
was the source of `ruzstd` and `zstd-safe`; spelling engines and selected grammar
rules remain tested.

The final audit is **not clean**, exits 1, and suppresses no advisories:

| Finding | Status |
| --- | --- |
| `zip` 7.2.0 and 8.6.0: RUSTSEC-2026-0367, -0368, -0369, -0429 | Eight records: extraction permission/symlink flaws and process abort. RustSec requires zip >=9.0.0, outside the pinned upstream requirements. zip 8.6.0 is active through Harper; zip 7.2.0 is only in the optional lockfile graph for this build. Tuipo does not expose ZIP import/extraction, but these findings remain open. |
| `lru` 0.16.4: RUSTSEC-2026-0253 | Active through Harper/harper-pos-utils; panic-safety memory unsoundness. Fixed >=0.18.2, outside their version constraints. Reachability of the necessary panicking key-drop path has not been proven here; do not assume this warning is harmless. |
| `bincode` 2.0.1: RUSTSEC-2025-0141 | Unmaintained; active via Harper's embedded model dependencies. |
| `paste` 1.0.15: RUSTSEC-2024-0436 | Unmaintained; used by the active build graph. |

An upstream upgrade or reviewed local backports are still needed before treating
this dependency baseline as ready for a security-sensitive release. No major-version
fork or blanket audit-ignore was introduced to hide these results. Full machine-readable
results are in [docs/dependency-audit.json](docs/dependency-audit.json). Advisory facts
come from [RustSec](https://github.com/RustSec/advisory-db); this is a dated snapshot,
not a guarantee about undisclosed or subsequently published vulnerabilities.

Recheck periodically (the developer tool itself can be installed into a temporary
local prefix; no global installation is needed):

```sh
cargo install cargo-audit --locked --root /tmp/tuipo-security-tools
/tmp/tuipo-security-tools/bin/cargo-audit audit --db /tmp/tuipo-rustsec-advisories
python3 scripts/check-local-policy.py
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

The audit downloads only public advisory/index data. It does not read prompts.
There is no audit invocation from the running spellchecker.

## Publication and automatic integration gate

The gate is **closed**. No fork, remote update, merge, push, shell function,
dotfile change, LaunchAgent or autostart activation is authorized by this review.
Known high-severity ZIP findings remain. Inspection of the current published
Harper 3.0.0 manifest also found `zip = "8.6.0"`; merely upgrading Harper is not a
complete fix. No unreviewed major-version dependency substitution was applied.

A raw-TUI application may request a token in the same stream used for ordinary
prose. There is no verified Codex API that marks safe prose for this wrapper, so
fully excluding secrets from an automatic PTY wrapper cannot currently be justified.
Password heuristics are not accepted as a publication/activation gate.
SSH-marked (`SSH_CONNECTION`, `SSH_CLIENT`, `SSH_TTY`) and nested (`TUIPO_ACTIVE`)
launches execute the selected app directly without checker processing; only the
presence of those variables is inspected. This does not make the selected app
itself confidential or prevent its network/history operations.

The historical repository scan found local home-path metadata in two earlier
`docs/VALIDATION.md` blobs and a historical `.DS_Store` blob. The current documentation
has been scrubbed, but history was not rewritten. These findings need an explicit
history/privacy decision before publication. Known credential signature scans found
no credential match; that finite scan does not prove there are no secrets or PII,
particularly in binary demo assets or commit identities.

The inherited release workflow contained write permissions, unpinned Actions,
`curl | sh` installers and an upstream-author Homebrew publication target. That
workflow and cargo-dist publication settings have been removed, rather than reused
for this unpublished fork. There are no remaining release/installer workflows.
Normal Cargo dependency build scripts still execute during developer builds;
`Cargo.lock` and source checksums are not a supply-chain sandbox.

See [the publication audit](docs/PUBLICATION_AUDIT.md),
[the integration design](docs/INTEGRATION.md), the committed dependency report and
license inventory. `gh auth status` succeeded, but authentication is not security
approval; no remote mutation was performed. User shell files may only be changed
after all gates, a complete exact diff and separate user confirmation.

## Evidence and reporting

Regression tests cover metadata logging with prompt/password/token/output canaries,
private-file restrictions, no input-history artifacts, bypass mode, disabled shell
installation, discarded partial UTF-8/stale lints and canonical no-echo PTY input.
Existing RU/EN, correction, passthrough and lifecycle tests remain in place. These
checks do not prove the absence of all exfiltration or all secret-field exposure.

If reporting an issue, include the version/commit, OS/app version and a synthetic
reproducer. Do not attach real prompts, credentials, old content logs, environment
dumps or private terminal recordings. No private security-reporting contact has
been established for this unpublished fork; do not post live secrets publicly.
