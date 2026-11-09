# Per-application Zsh integration: blocked design

No integration is installed, sourced or activated. `tuipo init` fails without
modifying dotfiles. `tuipo on` changes the mode for subsequent explicit launches;
it does not install a shell function or pass the security gate.

A future integration would define only a `codex` Zsh function in a separately
reviewed file. It would invoke fixed absolute paths for Tuipo and Codex, forward
arguments as `"$@"`, and use `command` to avoid recursive function lookup. It must
bypass wrapping in SSH, noninteractive/non-TTY sessions, other terminals, nested
wrappers and off/private mode. Ordinary commands must never pass through Tuipo.
PATH resolution must be reviewed at installation rather than trusted from writable
project directories. An environment flag or filename alone is not a trust boundary
against same-user malware.

Such a function would still pass all Codex input, including raw-mode password/API-key
widgets and secrets typed as prose, through the PTY. Codex does not supply this
wrapper with a verified nonsecret input boundary. Echo heuristics cannot satisfy the
requested exclusion of all secrets. Therefore **no executable automatic wrapper is
provided for activation**, even if dependency vulnerabilities are later fixed.
A reliable app/editor API limiting access to approved prose fields would be needed,
or the security requirements would need explicit reconsideration by the user.

The current program has no global keyboard monitoring, Accessibility API, Input
Monitoring permission, LaunchAgent, login item or shell-wide preexec hook. A PTY
proxy is started only with the chosen app and is tied to that app's lifetime; its
spell engine is an on-demand child. This differs from a background service that
runs independently and can observe multiple sessions. When `off` is observed, the
existing proxy forwards bytes until exit but stops input/screen reconstruction and
terminates the engine. `on` affects the next explicit launch, not an old disabled
proxy. A transient switch within an already-read batch is not an atomic cross-process
privacy boundary; use direct launch for sensitive sessions from the start.

If a safe integration becomes possible, before installation we must:

1. Pass the dependency, license, repository/history and secret-boundary gates.
2. Publish only after those gates, without force push or rewriting history.
3. Show the full exact file contents and the exact managed Zsh inclusion block,
   including absolute executable paths, ownership and permissions.
4. Obtain separate explicit user confirmation before modifying `.zshrc`, `.zprofile`
   or any user file. No LaunchAgent or daemon is needed.
5. Use an install manifest recording the managed block; refuse conflicting aliases
   or functions instead of overwriting them. On removal, delete only that exact
   unchanged block/file, preserve all surrounding user edits, then start a fresh
   terminal (or unload only the verified managed function).

There is no installation to uninstall in this checkout. Direct `codex` restores the
ordinary behavior. Do not run broad dotfile-rewriting or shell-history-cleaning
commands to remove a hypothetical integration.
