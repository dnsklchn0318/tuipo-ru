# Explicit file workflow

1. Draft text using your preferred editor.
2. Save a UTF-8 file and invoke `tuipo check -- file.txt`.
3. Review positions/suggestions and edit the file yourself.
4. Transfer the reviewed text to Codex yourself.

There is no automatic integration, clipboard access, shell hook, PTY proxy or
application launcher in this version. An App Server client or editor plugin would
be a separate project; neither is necessary for file checking. No Codex account,
API key or authentication-file access is needed by Tuipo.

The old `tuipo -- codex`, `init`, `on`, `off`, `status` and config TUI commands are
unsupported. Existing independently installed old binaries/hooks are not removed
by this source change. No user dotfile is inspected or rewritten.
