# RU/EN terminal checking

Harper checks English; Spellbook with the pinned LibreOffice dictionary checks Russian.
A background subprocess checks input snapshots; stale results cannot be applied to
new text. PTY forwarding, input buffering, echoed-screen matching and the original
picker/correction architecture are restored in version 0.4.0. The optional file
checker uses a separate internal subprocess mode.

Terminal.app uses plain underline. Type `превет world `, move into the typo and
press Ctrl+N; Ctrl+N/P cycles, Enter confirms, Escape cancels. Tab stays unchanged
unless legacy Tab-fix/picker is opted in. The menu requires a safe unoccupied screen
position and may pass shortcuts through if none is available. Complex graphemes,
wide input and earlier pasted lines suppress correction conservatively.

Manual check in Terminal.app: launch `tuipo -- codex`, type mixed RU/EN prose,
open/cycle/cancel/apply corrections, resize with a menu open, exercise Tab/history,
and exit normally. Automated tests validate the model and real PTY transport;
they do not certify every Codex footer layout or native terminal redraw.
