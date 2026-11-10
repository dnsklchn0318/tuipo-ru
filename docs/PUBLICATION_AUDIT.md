# Source publication review — 2026-11-10

Source preparation is separate from permission to install/run or distribute binaries.
No GitHub push, repository creation, release, installation or user-shell edit is
performed. Cargo repository/homepage metadata names the intended destination,
https://github.com/dnsklchn0318/tuipo-ru; it does not assert that it already exists.

| Decision | Result |
| --- | --- |
| Known ZIP/LRU vulnerabilities | Resolved: active ZIP 9.5.0, LRU 0.18.2; inactive Candle/ZIP 7 chain removed by direct Burn inference imports. No advisory ignores. |
| Maintenance warnings | Open and disclosed: bincode 2.0.1, paste 1.0.15. Strict warning gate fails. This is distinct from zero known vulnerability findings. |
| Codex secret interception | Removed production PTY/app-launch architecture. Only an explicitly selected file is checked; secrets in that file remain visible to the checker. |
| Current source privacy | Signature scanner finds no current matches; no secrets/PII absence guarantee. |
| Publication history | Separate sanitized copy: local home paths replaced, historical .DS_Store and obsolete binary demos removed, fork author/committer changed to GitHub login/noreply. Original upstream authorship retained. |
| Attribution/licenses | MIT/Apache upstream licenses, dictionary notice and Spellbook MPL retained. Modified Harper packages carry Apache text, original manifests and reviewable patches. SPDX inventory has no missing declarations. Embedded model redistribution has not received independent legal certification. |
| Source publication readiness | Prepared for review in the separate sanitized copy, with documented maintenance risks. No confidential-session certification is required or claimed. |
| Binary release | Not prepared: supported-platform validation and a complete binary notice bundle remain separate work. |
| Installation/run | Not executed. File selection explicitly exposes that file to the checker; no automatic integration exists. |

## History handling

A private bundle preserves original refs. The original checkout/refs/remotes are
unchanged; implementation edits remain in its working tree. The publication clone
has no remote configured. It retains upstream commits and modified descendant
commits; hashes necessarily change after sanitization. Old report commit references
may refer to the original history and are historical evidence, not current heads.

Only the upstream author's already public identity and the maintainer's GitHub
login/noreply identity are permitted by the scanner. It emits object IDs/rule names,
not discovered identity values. Historic binary demos are omitted entirely, so no
visual privacy certification of those images is needed for this publication set.
Private backup/tool/publication directories are excluded from Git and packaging.
The committed repository-audit.json applies to the sanitized copy; an external
repeat checks the final report-only commit. It is not evidence that the original
local history has been erased.

## Evidence

See VALIDATION.md, DEPENDENCY_REACHABILITY.md, dependency-audit.json,
dependency-licenses.json, repository-audit.json and vendor/README.md.
No blanket claim of secure execution, complete supply-chain review or legal
certification follows from these checks. Source publication can proceed as an
experimental file checker with the disclosed maintenance limitations; actual remote
publication remains a separate action.
