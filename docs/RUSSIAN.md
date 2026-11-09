# RU/EN implementation and manual validation

This fork extends the upstream Tuipo architecture; there is no second picker,
second input buffer, cloud spellchecker or Russian grammar engine.

| Component | Decision |
| --- | --- |
| `spell.rs` | Keep Harper's English rules/ranking; route Russian spans to Spellbook 0.4.2 with the pinned LibreOffice UTF-8 aff/dic pair. Suppress Harper lints touching Russian. Share the parsed bundled Russian dictionary once per process. Disable expensive n-gram suggestions, retain ordinary Hunspell edit suggestions. |
| `engine.rs`, `lint_worker.rs` | Retain framed subprocess protocol, async snapshots, epoch/stale-result rejection, lifecycle and failure handling. Engine now loads the same TOML config. Dictionary exceptions remain on the client. |
| `buffer.rs`, `input.rs` | Reuse byte-boundary cursor, UTF-8 assembly, scalar arrow/Backspace handling, bracketed/burst paste and snapshot model. |
| `pty.rs` | Add configurable completion keys; use only immediately available exact-text lints. Renderer shares menu eligibility for exact text/cursor. Preserve PTY forwarding on failure. Dismiss selection on editing/paste and invalidate on resize. Emit complete Unicode echo events. |
| `picker.rs` | Reuse cyclic selection, Enter/Escape and overlay restore. Carry exact span offsets to distinguish repeated words. Reject wrapped/ambiguous placement and occupied overlay cells. |
| `fix.rs` | Reuse arrow/backspace injection. Validate exact byte/scalar spans and live word. Restrict injected edits to one-cell, one-scalar graphemes; reject controls/newlines in replacements and active paste. |
| `screen.rs`, `echo.rs`, `paint.rs` | Preserve grid anchoring and underline style. Track ordinary wide scalars in screen cursor advancement; suppress input overlays on non-unit-width content. Honor `pause_ms` in actual painting. |
| `config.rs`, `config_tui.rs` | Keep old TOML parameters and TUI; serialize new settings so TUI saving preserves them. New fields are edited in TOML. Validate configured dictionary pairs and language names before entering raw mode. |

Harper's existing heuristics and grammar opt-in remain available. Code-shaped
whitespace tokens, URLs, paths, flags, identifiers, bundled CLI exceptions and
backtick-enclosed fragments are skipped. The filter is heuristic: an unknown
standalone executable can still resemble a typo. Russian inflections are checked
by affix rules; this does not provide contextual disambiguation of е/ё, real-word
errors, grammar or a full proper-name lexicon.

Upstream screen/buffer inference cannot observe the wrapped application's actual
editor state. History completion, mouse editing, custom readline bindings, hidden
paste summaries and app-specific cursor movement can desynchronize it. Painting
checks echoed grid text, but this is not an editor API and cannot guarantee safe
editing in every TUI. Ordinary composed Russian/English words are supported;
combining sequences, emoji, ZWJ sequences and CJK **inside input** disable injected
fixes and input overlays conservatively. This is an explicit limitation, not full
grapheme editing support. The screen observer handles ordinary scalar widths but
is not a complete terminal emulator for all emoji sequences.

Completed words are underlined after a separator and an idle pause (150 ms by
default); the final unseparated word is not painted while typing. It can still be
corrected at its end with the explicit shortcut once its lint is ready. Russian
misspellings without suggestions may be underlined but cannot open a menu.
A word on an earlier pasted line cannot be fixed; this retains the safeguard
against injecting newline submit keys. A wrapped word may be underlined, while
its menu is suppressed when the current single-row layout cannot place it safely.

The menu never overwrites known nonblank host cells; this can deliberately make
it unavailable in a crowded Codex footer or a narrow terminal. Position approval
is asynchronous: if there is no safe recent position, the shortcut passes through.
A host redraw between approval and drawing can still race the overlay. Restore
uses upstream grid characters and resets SGR, so exact background/text attributes
are not guaranteed. Native Terminal.app scrolling, Codex redraws and Zsh editing
must be evaluated manually before claiming full compatibility. Plain underline
is selected for `TERM_PROGRAM=Apple_Terminal`; `underline = "plain"` forces it.

Recorded automated and Zsh PTY results: [VALIDATION.md](VALIDATION.md).

## Terminal.app manual checks

Run from the repository; these commands do not install anything or edit dotfiles:

```sh
cargo build --release
./target/release/tuipo -- codex
# Separately:
./target/release/tuipo -- zsh
```

1. Type `Привет world, работающие люди читают книги. `, wait for the pause;
   verify no spelling underlines on correct Russian inflections or English.
2. Type `превет world ашибка `; wait. Move Left into each typo or immediately
   after it, press Ctrl+N. Verify text stays unchanged until Enter. Cycle forward
   and backward through all candidates (wrap at ends). Escape must preserve text.
3. Confirm lowercase/titlecase/uppercase `превет / Превет / ПРЕВЕТ`. Test `елка`,
   `ёлка`, `всё`, `все`; no automatic е/ё changes. Add jargon to a temporary
   exception file and point `dict_path` at it; restart and verify the exception.
4. Correct first/middle/last words; choose longer and shorter suggestions. Verify
   suffix characters are preserved and cursor lands after the replacement for a
   middle edit, or at the restored line end for an end edit.
5. With correct text or no suggestions, verify Ctrl+N/P reach Codex/Zsh. Test Tab,
   Shift+Tab, host completion/history and ordinary typing before/after dismissing
   a menu. `highlight_only = true` must pass all editing shortcuts through.
6. Paste mixed multiline text, including URLs, paths, flags, identifiers and code.
   No menu keys should run during bracketed paste. Check edits after paste. Earlier
   pasted lines must not be changed by correction. Emoji/combining/CJK input must
   suppress correction rather than corrupt surrounding text.
7. Resize while a menu is open, shrink width until a word wraps, scroll and cause
   a host redraw. Verify no stale selection is applied and no random menu is drawn.
   Menu may disappear/pass through if no safe position exists. Inspect cleanup,
   prompt/footer contents and cursor after each Enter/Escape.
8. Exit Codex/Zsh normally; verify terminal echo and cursor remain usable.

Record actual app versions and observed failures. Automated tests cover model
behavior and real PTY passthrough/engine lifecycle, not native GUI visual fidelity.

## Licensing

[Dictionary provenance](../assets/ru_RU/SOURCE.md) pins exact upstream files and
links license notices. Spellbook is MPL-2.0; the dictionary has its own permissive
license. Both are compatible with keeping this fork's existing MIT/Apache source
licenses, provided their separate notices and redistribution terms are retained.
Ship README and `assets/ru_RU` notices with binary release artifacts. No upstream
license or author attribution was removed. Spellbook source is available through
its Cargo package and the linked upstream repository.

## Updated operating mode

See [README](../README.md), [SECURITY](../SECURITY.md) and
[the publication audit](PUBLICATION_AUDIT.md) for the current controls. `off` now
ends input/screen reconstruction in a running proxy permanently until app exit;
`on` affects the next explicit launch. SSH and nested launches bypass processing.
Persistent engine failure disables checking instead of falling back in-process.
Automatic Codex integration and publication remain blocked.
