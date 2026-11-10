# Explicit terminal launch

Primary workflow: run `tuipo -- codex` in Terminal.app. The wrapper starts Codex
in the current directory, forwards its arguments and checks typed RU/EN input.
Checking cannot be attached to a Codex process already running.

No shell-profile editing, GUI permission, service or global installation is needed.
The prepared binary can be invoked by absolute path from any project directory.
Ctrl+N/P opens/cycles suggestions at the cursor; Enter confirms; Escape dismisses.
Tab retains application completion by default.

`codex` alone runs without Tuipo. `off` disables processing and subsequent wrapping;
`on` affects a new application launch. The optional `check -- file` subcommand
remains available for file checks. No Codex settings or account files are modified.
