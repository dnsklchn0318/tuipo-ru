# Dependency reachability review — 2026-11-10

Baseline: upstream Harper core/pos-utils/brill 2.0.0; original lockfile at d1d4c04.
Scope: source inspection of application, active normal/build dependencies across
all targets, and inactive lockfile packages. This is not exhaustive fuzzing or a
formal proof. No advisory ignore is configured.

| Advisory | Preconditions / original route | Application reachability | Resolution |
| --- | --- | --- | --- |
| RUSTSEC-2026-0367 | ZIP extraction preserves SUID/SGID from attacker attributes; elevated extraction amplifies impact | Harper's only ZIP consumer is Weirpack::from_bytes/by_index plus writer; no archive extraction in app or Harper. User text and Hunspell dictionaries never enter a ZIP reader. No app call to Weirpack import. | Active ZIP 8.6 -> 9.5.0; fixed baseline >=9.0.0 |
| RUSTSEC-2026-0368 | extract creates symlink with external target; later access follows it | Same absent extraction path; merely parsing a ZIP directory is not the vulnerable filesystem operation | Same update |
| RUSTSEC-2026-0369 | archive/stream extract chmod follows symlink outside destination | Neither extraction API is called by Harper/app; no archive output directory | Same update |
| RUSTSEC-2026-0429 | extract allocates symlink target using declared uncompressed size; abort is not catchable as panic | Same absent extract path. No untrusted archive import exposed. This does not establish safety of every ZIP parser operation | Same update; upstream fix zip-rs/zip2#984 |
| RUSTSEC-2026-0253 | LruCache::pop drops key before detaching; key Drop panics, unwind is caught, cache is subsequently reused | Harper suggestion cache uses get/put (CharString); lint cache get/put ((u64,u64)); AST cache peek/put ((Arc<String>,bool)); CachedChunker get/put (Vec<String>, Vec<Option<UPOS>>). No LRU pop invocation; these key destructors have no application-defined panicking Drop. Vec/String pop matches are unrelated. App accepts text, not custom Rust key implementations. Eviction internals were also inspected; removal/eviction detaches before key disposal. Original worker catch_unwind alone did not satisfy all preconditions. | Both consumers now use 0.18.2; detach-before-drop implementation verified; panic/drop regression exercises subsequent cache eviction |

Conclusions: the described ZIP extraction and LRU panic-key exploit paths were not
found reachable from this application's supported inputs. These conclusions are
specific to the inspected versions, types and call paths; updates still remove the
known defective code instead of treating non-reachability as an audit waiver.

## Inactive ZIP and narrowed dependency graph

Original ZIP 7.2.0 was absent from active builds but resolved through the Burn
umbrella's optional Candle backend. harper-pos-utils now imports burn-core and
burn-nn directly; training-only imports use burn-autodiff/burn-optim. Inference
algorithm, model weights and harper-brill version remain unchanged. The umbrella,
Candle and ZIP 7.x no longer occur in Cargo.lock. This removes an unused capability;
it does not relabel a vulnerable package or suppress a report. Both normal/build
all-target policy and full lockfile audit are rerun.

## Verification and remaining warnings

Fresh audit fetched RustSec DB d9cb059f3217db87aeccd190b81eaffe8cc29cb9,
1,430 advisories, last updated 2026-11-09. cargo-audit 0.22.2 reports zero
vulnerability records and no unsound warning. bincode 2.0.1 (RUSTSEC-2025-0141,
Burn model serialization) and paste 1.0.15 (RUSTSEC-2024-0436, macro dependency)
remain unmaintained. They have not been silently replaced with API-incompatible
implementations. `--deny warnings` exits 1; ordinary audit exits 0. The sandboxed
run cannot check the crates.io yanked index lock; a privileged repeat is recorded
in validation. Future undisclosed vulnerabilities are not covered.

Sources: [LRU advisory](https://rustsec.org/advisories/RUSTSEC-2026-0253.html),
[LRU fix](https://github.com/jeromefroe/lru-rs/pull/238),
[ZIP permissions](https://github.com/zip-rs/zip2/security/advisories/GHSA-vv5m-jxxh-4627),
[ZIP symlinks](https://github.com/zip-rs/zip2/security/advisories/GHSA-4v9h-p7m9-rh26),
[ZIP chmod](https://github.com/zip-rs/zip2/security/advisories/GHSA-4684-h2q9-2vcp),
[ZIP abort fix](https://github.com/zip-rs/zip2/pull/984).
