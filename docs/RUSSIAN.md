# RU/EN implementation

Harper 2.0.0 checks English; Spellbook with the pinned LibreOffice aff/dic pair
checks Russian. Both share case-insensitive exception filtering and heuristic
code/URL/path filtering. Russian grammar and contextual е/ё disambiguation are not
provided. Suggestion ranking is not a guarantee of intended meaning.

The parent opens the selected regular UTF-8 file, enforces 1 MiB, and sends its
contents through a pipe to a short-lived local checker. The checker loads read-only
configuration/dictionaries and returns JSON findings. The parent prints escaped
words/suggestions and line/scalar-column positions. There is no terminal-state
inference or text replacement. See VALIDATION.md and SECURITY.md.
