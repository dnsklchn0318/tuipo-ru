# Version 0.3.0 preparation

Source repository: https://github.com/dnsklchn0318/tuipo-ru

This version checks an explicitly selected UTF-8 file locally. It replaces the
old terminal wrapper. Version 0.3.0 distinguishes the fork from upstream Tuipo 0.2.0.

A local macOS ARM64 archive is prepared from this source using:

```sh
cargo build --locked --offline --release
./tuipo check -- draft.txt
```

The prepared archive contains the executable, instructions, checksum, source
snapshot, source packages of dependencies and their license declarations, original
licenses/notices and Russian dictionary notices. Build paths are remapped for the
distribution binary. No installation or shell integration is performed.

The archive is prepared locally; it is not a GitHub binary release. Version tags
inherited from upstream describe upstream history and are not releases of this fork.
