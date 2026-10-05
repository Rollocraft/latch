# Latch

**The control layer for autonomous software.**

[![CI](https://github.com/Rollocraft/latch/actions/workflows/ci.yml/badge.svg)](https://github.com/Rollocraft/latch/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
![Rust 1.98+](https://img.shields.io/badge/rust-1.98%2B-orange.svg)

Latch is a vendor-neutral security and runtime layer for AI agents. It sits
between an agent and the systems it touches and decides what the agent may
access, change, execute or send — and keeps a tamper-evident record of what
it did.

```text
            AI agent
               │  semantic action: git.push, file.write, …
               ▼
 ┌──────────────────────────────┐
 │            Latch             │  identity · policy · approvals · budgets
 │                              │  transactions · egress control · audit
 └──────────────────────────────┘
               │  only what was allowed, staged where possible
               ▼
   filesystem · git · network · …
```

Agent actions under Latch are:

- **controlled** — default deny, explicit rules, argument-aware decisions
- **attributable** — every action is bound to an agent identity, a session and a human owner
- **reviewable** — changes are staged and shown before they reach the host
- **reversible where possible** — committed changes can be rolled back byte for byte
- **recorded** — a hash-chained audit log that detects tampering and reordering

## Key features

### Policy engine
- Deterministic, **default-deny** evaluation with a fixed precedence:
  *deny* beats *ask* beats *allow*; *limit* attaches a budget to an allowed action.
- Hierarchical policy levels (`company`, `department`, `team`, `agent`, `session`) —
  a session cannot opt out of a company rule.
- **Argument-aware rules**: `git push` and `git push --force` are the same
  program but not the same action.
- Human-readable **JSON or YAML** policy documents with strict validation
  (unknown fields are rejected, inputs are size-bounded).
- **Offline policy testing**: run scenario files against a policy in CI, and
  explain any hypothetical decision rule by rule.

### Filesystem transactions
- Agents write into a private **copy-on-write workspace**; the host is
  untouched until someone commits.
- **Reviewable diffs** (new / modified / deleted / renamed) before anything lands.
- **Partial commit** of selected paths.
- **Exact rollback** after commit — contents, permissions, directory structure
  and symlink targets are journaled before they are replaced.
- Workspaces are durable on disk and can be resumed by another process.

### Execution gate and approvals
- One execution path checks policy, approvals and budgets together; action IDs
  are consumed before execution so an ambiguous retry can never repeat an effect.
- **Approvals** with expiry, revocation and **multi-approver quorums** of
  distinct principals.
- **Capability tokens**: random, scoped, short-lived grants that are audited on
  issue and use.
- **Budgets**: atomic reservations with overflow-safe accounting, including a
  multi-currency finance ledger for spend limits.

### Network egress control
- Decisions bind to the runtime's **own DNS resolution**, never to the string an
  agent supplied — direct-IP access, CNAME laundering and DNS rebinding are
  closed by construction.
- TTLs are clamped to ten minutes; private, link-local and cloud-metadata
  addresses (`169.254.169.254`) are denied by default.
- Per-session **egress budgets** (bytes, destinations, requests) and a
  deterministic **exfiltration monitor** (secret read followed by an unfamiliar
  upload, or a large burst leaving at once).

### Audit
- Append-only, **hash-chained** audit segments, fsynced on every append;
  `latch audit` verifies the chain and prints its head for external checkpointing.
- Tenant-isolated queries, bounded pagination, session timelines and
  compliance counts.
- JSONL export with configurable resource redaction.

### Identity, risk and manifests
- Agent identities with organization, team, owner, purpose, model and validity
  window; sessions with an explicit lifecycle.
- **Shell commands become semantic actions** (`git.force_push`,
  `package.install`, `command.shell`, …), each with a reversibility class.
- An explainable **risk score** with a per-factor breakdown.
- **Agent manifests** declare the permissions an agent needs; `latch manifest diff`
  shows exactly which permissions a new version adds, removes or widens.

### Supervision and presentation
- A sandbox lifecycle state machine with explicit assurance levels and resource
  limits, and an organization-scoped supervisor with a **kill switch** that
  reports every termination individually.
- Terminal rendering for approvals, timelines and transaction review that is
  byte-bounded and escapes control characters, so agent-controlled text cannot
  forge what a human reviewer sees.

## Scope of 1.0

Latch 1.0 ships the decision, transaction and audit layers as tested libraries
and a `latch` CLI for policies, transactions, manifests and audit verification.

It does **not** yet ship an OS confinement backend. Latch mediates what goes
*through* it; an agent process with direct access to the filesystem or network
can bypass it, so run agents under OS-level isolation as well. Accordingly,
`latch run` refuses to start a process, and `latch capabilities` lists exactly
what is and is not available.

## Installation

Download a prebuilt binary for Linux, macOS or Windows from the
[latest release](https://github.com/Rollocraft/latch/releases/latest), or build
from source (the toolchain is pinned by `rust-toolchain.toml`):

```sh
git clone https://github.com/Rollocraft/latch.git
cd latch
cargo install --path crates/latch-cli
```

## Usage

```text
latch policy validate <file>                       validate a JSON or YAML policy
latch policy explain <policy> <action> [resource]  explain a hypothetical decision
latch policy test <policy> <scenarios>             run offline scenario tests
latch tx begin <root> <workspace>                  open a transaction over a directory
latch diff <workspace>                             show staged changes
latch commit <workspace> [path...]                 apply all staged changes, or only these
latch rollback <workspace>                         undo the transaction
latch status <workspace>                           read-only transaction state
latch manifest check <file>                        validate an agent manifest
latch manifest diff <old> <new>                    show requested permission changes
latch audit <segment-file>                         verify and print an audit segment
latch capabilities                                 what this build can and cannot do
```

Run `latch --help` for the full reference.

### Policies

A policy is a list of rules. Each rule selects an action, a resource, an
environment and an actor (`any`, `exact` or `prefix`), optionally matches the
action's arguments, and has one effect: `allow`, `deny`, `ask`, `limit` or `log`.

```yaml
version: 1
policies:
  - id: default
    level: company
    rules:
      - id: review-pushes
        description: Pushing changes leaves the machine and needs a human.
        action: { kind: exact, value: git.push }
        resource: { kind: any }
        environment: { kind: any }
        actor: { kind: any }
        effect: { kind: ask }

      - id: no-force-push
        description: History rewrites on shared branches are never allowed.
        action: { kind: exact, value: git.push }
        resource: { kind: any }
        environment: { kind: any }
        actor: { kind: any }
        arguments: { kind: any_flag, values: ["--force", "-f"] }
        effect: { kind: deny }
```

Test a policy against expected outcomes before rolling it out:

```console
$ latch policy test examples/policy.yaml examples/scenarios.json
PASS "read-source": Allow; mismatches=[]
PASS "read-secrets": Deny; mismatches=[]
PASS "read-outside-project": Deny; mismatches=[]
PASS "write-source": Limited; mismatches=[]
PASS "delete-source": Deny; mismatches=[]
PASS "push": ApprovalRequired; mismatches=[]
PASS "force-push": Deny; mismatches=[]
7 passed; 0 failed
```

The command exits non-zero on any mismatch, so it drops straight into CI.
See [`examples/`](examples) for the complete policy, scenarios and manifests.

### Transactions

```console
$ latch tx begin ./project ./workspace
$ # … the agent writes into ./workspace/stage …
$ latch diff ./workspace
new       "notes.txt"
modified  "readme.txt"

1 modified file
1 new file
$ latch commit ./workspace notes.txt     # commit only what you approve
committed
1 new file
$ latch rollback ./workspace             # and undo it exactly if needed
rolled back
```

### Manifests

```console
$ latch manifest diff examples/manifests/coder-v1.json examples/manifests/coder-v2.json
…
requires_review: true; requested permissions only, not grants
```

## Architecture

Latch is a Cargo workspace of small, focused crates:

| Crate | Responsibility |
| --- | --- |
| [`latch-core`](crates/latch-core) | Actions, identities, sessions, command classification, risk, finance, SHA-256, control protocol |
| [`latch-policy`](crates/latch-policy) | Policy documents, deterministic evaluation, dry runs, scenario tests |
| [`latch-runtime`](crates/latch-runtime) | Execution gate, approvals and quorums, capability tokens, budgets |
| [`latch-transaction`](crates/latch-transaction) | Copy-on-write filesystem transactions, journaled commit and rollback |
| [`latch-fs`](crates/latch-fs) | Filesystem action adapter executing allowed actions through a transaction |
| [`latch-net`](crates/latch-net) | DNS-bound egress policy, egress budgets, exfiltration monitor |
| [`latch-audit`](crates/latch-audit) | Hash-chained audit segments, queries, timelines, JSONL export |
| [`latch-agents`](crates/latch-agents) | Agent manifests, identity binding, permission diffs |
| [`latch-substrate`](crates/latch-substrate) | Enforcement vocabulary: assurance levels, backend capabilities, limits |
| [`latch-sandbox`](crates/latch-sandbox) | Sandbox lifecycle state machine over a pluggable backend |
| [`latch-warden`](crates/latch-warden) | Organization-scoped supervision and kill switch |
| [`latch-ipc`](crates/latch-ipc) | Size-bounded, length-prefixed JSON framing |
| [`latch-tui`](crates/latch-tui) | Bounded, escaped terminal rendering for human review |
| [`latch-cli`](crates/latch-cli) | The `latch` command-line tool |

Design decisions are recorded as ADRs in [`docs/adr`](docs/adr).

## Security model

- **Fail closed.** Unknown fields, oversized inputs, missing policies, expired
  identities and unresolved destinations are denials, not warnings.
- **The agent is untrusted.** Policy, approvals and budgets are evaluated by
  trusted runtime components; nothing the agent supplies is taken as authority.
- **Determinism.** The same policy and the same action always produce the same
  decision, so every decision can be replayed and explained.
- **Honest guarantees.** Each crate documents what it does *not* enforce. In
  particular, mediation is not confinement — see [Scope of 1.0](#scope-of-10).

## Development

```sh
cargo test --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

CI runs the test suite on Linux, macOS and Windows.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
