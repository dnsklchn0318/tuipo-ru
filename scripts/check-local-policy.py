#!/usr/bin/env python3
"""Regression guard for the local-only spellchecker; not an OS network sandbox."""
from pathlib import Path
import re
import subprocess
import sys

root = Path(__file__).resolve().parent.parent
result = subprocess.run(
    ["cargo", "tree", "--locked", "--offline", "--target", "all", "-e", "normal,build", "--prefix", "none"],
    cwd=root, text=True, capture_output=True, check=False,
)
if result.returncode:
    sys.exit(result.stderr)
packages = set(re.findall(r"^([\w-]+) v", result.stdout, re.MULTILINE))
forbidden = {
    "reqwest", "ureq", "hyper", "hyper-util", "curl", "curl-sys", "sentry",
    "opentelemetry", "opentelemetry_sdk", "segment", "amplitude", "mixpanel",
    "quinn", "quinn-proto", "tokio-tungstenite", "tonic",
}
violations = [f"active network/telemetry dependency: {name}" for name in sorted(packages & forbidden)]
for path in sorted((root / "src").glob("*.rs")):
    source = path.read_text()
    for token in ["std::net::", "tokio::net::", "reqwest::", "ureq::", "sentry::", "opentelemetry::"]:
        if token in source:
            violations.append(f"network/telemetry entry point in {path.name}: {token}")
if violations:
    sys.exit("\n".join(violations))
print(f"Local-only policy guard passed ({len(packages)} active normal/build packages, all targets).")
