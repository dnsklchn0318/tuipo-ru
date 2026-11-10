# Reviewed Harper adaptations

Sources are crates.io harper-core 2.0.0 and harper-pos-utils 2.0.0, Apache-2.0,
https://github.com/Automattic/harper. Original source attribution is retained.
Upstream crate-local lockfiles are omitted; the root lockfile is authoritative.
Cargo.toml.orig records upstream manifests; Cargo.toml is the effective manifest.

Changes:
- Pin harper-brill to the tested 2.0.0 model baseline.
- Use ZIP 9.5.0 (RustSec fixes >=9.0.0) and propagate fallible UTF-8 archive names.
- Use LRU 0.18.2 (RUSTSEC-2026-0253 fix); construct the AST cache with an explicit
  hashbrown 0.16 hasher, because LRU now defaults to hashbrown 0.17.
- harper-pos-utils uses burn-core and burn-nn directly instead of the umbrella
  burn facade. Tensor/model code is unchanged. Training-only imports use optional
  burn-autodiff/burn-optim. This removes unused backend dependencies, including
  Candle's ZIP 7.x, from the lockfile rather than suppressing advisories.

These are modified local packages, not upstream Harper releases. See
../docs/DEPENDENCY_REACHABILITY.md and ../docs/VALIDATION.md for validation.
