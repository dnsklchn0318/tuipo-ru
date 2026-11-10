# Third-party notices

This fork retains the original Tuipo licenses and attribution:
[LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE).
Original project: https://github.com/ARahim3/tuipo by ARahim3.
No claim is made that this fork originally developed Tuipo's architecture.

Direct spell engines:

| Component | Version | License | Source |
| --- | --- | --- | --- |
| Harper core | 2.0.0 | Apache-2.0 | https://github.com/Automattic/harper |
| Spellbook | 0.4.2 | MPL-2.0 | https://github.com/helix-editor/spellbook |
| Russian aff/dic | pinned LibreOffice commit | separate permissive license below | [provenance](assets/ru_RU/SOURCE.md) |

Harper core and harper-pos-utils 2.0.0 are locally modified Apache-2.0 packages;
see vendor/README.md and their included LICENSE-APACHE files.

Spellbook is used unmodified. Its MPL text is included in
[assets/ru_RU/SPELLBOOK-MPL-2.0.txt](assets/ru_RU/SPELLBOOK-MPL-2.0.txt).
Cargo.lock fixes dependency versions and registry checksums. The active dependency
license inventory is [docs/dependency-licenses.json](docs/dependency-licenses.json).
An inventory is not a substitute for reviewing redistribution obligations when
creating binary artifacts. No binary release is authorized by this review.

Redistributions containing the bundled Russian dictionary must ship its complete
notice below (also included beside the dictionary). Other components retain their
respective notices in Cargo source packages. A future binary release must include
this file, Tuipo's two licenses and the dictionary/Spellbook notices.

## Russian dictionary notice (verbatim)

```text
* Copyright (c) 1997-2008, Alexander I. Lebedev

Modifications:
------------------------------------------------------------------
2012-08-24: Laszlo Nemeth (nemeth at numbertext org)
* ru_RU.aff: add TRY line for better suggestions
  (fix fdo#35001, reported by sasha.libreoffice at gmail)

Copyright:
------------------------------------------------------------------
* Copyright (c) 1997-2008, Alexander I. Lebedev

All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
* Redistributions of source code must retain the above copyright
  notice, this list of conditions and the following disclaimer.
* Redistributions in binary form must reproduce the above copyright
  notice, this list of conditions and the following disclaimer in the
  documentation and/or other materials provided with the distribution.
* Modified versions must be clearly marked as such.
* The name of Alexander I. Lebedev may not be used to endorse or promote
  products derived from this software without specific prior written
  permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.

```
