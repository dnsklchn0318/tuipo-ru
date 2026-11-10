# Changes

## Unreleased — explicit file checking

- Replace terminal interception with `tuipo check -- <file>`; no automatic Codex
  integration, clipboard access or text injection.
- Preserve Russian/English checking, dictionary exceptions and suggestion ranking.
- Update ZIP/LRU through documented local Harper adaptations and remove unused
  backend dependencies. Maintenance warnings remain visible.
- Prepare a separately sanitized publication history while preserving upstream
  authorship and original licenses. No remote publication or installation.

Earlier Tuipo 0.2.0 terminal features are preserved in upstream Git history and are
not commands supported by this version.
