#!/usr/bin/env python3
"""Offline heuristic scan of tracked files and all reachable Git blobs.

Prints paths/rule names, never matching secret values. Not a proof of no secrets.
Run against the local repository before considering publication.
"""
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent

def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT)

RULES = {
    "private-key": rb"-----BEGIN (?:RSA |EC |OPENSSH |DSA |ENCRYPTED )?PRIVATE KEY-----",
    "github-token": rb"\b(?:gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{40,})\b",
    "aws-access-key": rb"\b(?:AKIA|ASIA)[A-Z0-9]{16}\b",
    "provider-token": rb"\bsk-(?:proj-|ant-)?[A-Za-z0-9_-]{32,}\b",
    "slack-token": rb"\bxox[baprs]-[A-Za-z0-9-]{24,}\b",
    "credential-url": rb"https?://[^\s/:]+:[^\s/@]{4,}@",
    "local-home-path": rb"/Users/[A-Za-z][A-Za-z0-9_.-]{2,}/",
}
RULES = {name: re.compile(pattern) for name, pattern in RULES.items()}

def forbidden_path(path):
    parts = Path(path).parts
    name = Path(path).name
    return (
        name in {"AGENTS.md", "CLAUDE.md", "GEMINI.md", ".env", ".DS_Store"}
        or name.startswith(".env.")
        or any(part in {".codex", ".agents", ".claude", ".aws", ".ssh", "target"} for part in parts)
        or name.endswith((".pem", ".key", ".p12", ".pfx", ".log"))
    )

def findings(data):
    return [name for name, rule in RULES.items() if rule.search(data)]

tracked = git("ls-files", "--cached", "--others", "--exclude-standard", "-z").decode().split("\0")[:-1]
current = []
for path in tracked:
    file = ROOT / path
    if file.is_symlink():
        current.append({"path": path, "rules": ["symlink-needs-review"]})
        continue  # never follow a candidate link outside the repository
    if not file.exists():
        continue  # local deletion prepared for the next commit
    rules = findings(file.read_bytes())
    if forbidden_path(path):
        rules.append("forbidden-local-file")
    if rules:
        current.append({"path": path, "rules": rules})

objects = []
for line in git("rev-list", "--objects", "--all").decode().splitlines():
    oid, _, path = line.partition(" ")
    objects.append((oid, path))
process = subprocess.Popen(["git", "cat-file", "--batch"], cwd=ROOT,
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE)
history = []
count = 0
for oid, path in objects:
    process.stdin.write((oid + "\n").encode())
    process.stdin.flush()
    header = process.stdout.readline().split()
    size = int(header[2])
    data = process.stdout.read(size)
    process.stdout.read(1)
    if header[1] != b"blob":
        continue
    count += 1
    rules = findings(data)
    if forbidden_path(path):
        rules.append("historical-local-file")
    if rules:
        history.append({"object": oid, "path": path, "rules": rules})
process.stdin.close()
process.wait()
# Commit identities are public data too. Retain the upstream author's established
# public identity and the fork's GitHub-only identity; emit rules, never values.
metadata = []
for oid in git("rev-list", "--all").decode().splitlines():
    raw = git("cat-file", "commit", oid).split(b"\n\n", 1)[0]
    for line in raw.splitlines():
        if line.startswith((b"author ", b"committer ")):
            identity = line.split(b" ", 1)[1].rsplit(b">", 1)[0] + b">"
            if identity not in {
                b"ARahim3 <erahim3@gmail.com>",
                b"dnsklchn0318 <354221970+dnsklchn0318@users.noreply.github.com>",
            }:
                metadata.append({"object": oid, "rules": ["unreviewed-commit-identity"]})
report = {
    "scope": "working tracked files and blobs reachable from local refs; no remote scan",
    "method": "known credential signatures, forbidden paths and local home paths; no entropy or manual visual proof",
    "reviewed_head": git("rev-parse", "HEAD").decode().strip(),
    "commits": int(git("rev-list", "--count", "--all")),
    "candidate_paths": len(tracked), "historical_blobs": count,
    "current_findings": current, "historical_findings": history,
    "metadata_findings": metadata,
}
print(json.dumps(report, indent=2))
sys.exit(1 if current or history or metadata else 0)
