# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses
[Semantic Versioning](https://semver.org/).

## [1.0.0] - 2026-10-05

First public release.

### Added

- **Policy engine** — deterministic, default-deny evaluation over JSON and YAML
  policy documents with hierarchical levels, argument-aware rules, budgets
  (`limit`), approvals (`ask`), dry runs and offline scenario tests.
- **Filesystem transactions** — copy-on-write workspaces with reviewable diffs,
  partial commit and exact, journaled rollback after commit.
- **Execution gate** — policy, approvals and budgets checked in one path;
  single-use action IDs; approval expiry, revocation and multi-approver quorums;
  scoped, audited capability tokens; budget and finance ledgers.
- **Network egress control** — decisions bound to the runtime's own DNS
  resolution, CNAME chain evaluation, TTL clamping, private-address denial,
  egress budgets and an exfiltration monitor.
- **Audit** — hash-chained, fsynced audit segments with chain verification,
  tenant-isolated queries, timelines, compliance counts and redacting JSONL export.
- **Identity and manifests** — agent identities and sessions, command
  classification into semantic actions, explainable risk scores, agent
  manifests and permission diffs.
- **Supervision** — sandbox lifecycle state machine, organization-scoped
  supervisor with kill switch, bounded and escaped terminal rendering.
- **`latch` CLI** — `policy validate|explain|test`, `tx begin|changes|commit|rollback`,
  `status`, `diff`, `commit`, `rollback`, `manifest check|diff`, `audit`,
  `capabilities`.
- Example policy, scenarios and manifests under `examples/`.
- Prebuilt binaries for Linux (x86_64), macOS (Apple silicon and Intel) and
  Windows (x86_64).

### Known limitations

- No OS confinement backend is included; `latch run` refuses to start a process.
  Latch mediates actions that go through it and must be combined with OS-level
  isolation for untrusted agent processes.
- Budget accounting in the execution gate is process-local.

[1.0.0]: https://github.com/Rollocraft/latch/releases/tag/v1.0.0
