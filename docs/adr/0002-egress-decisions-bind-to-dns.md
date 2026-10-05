# ADR 0002: Egress decisions bind to the runtime's own name resolution

Status: accepted · Supersedes: none

## Context

Network policies are written about names — `github.com`, `*.unknown.dev` — but
a socket connects to an address. Every gap between those two is a way around
the policy. The known gaps are direct IP access, redirects, CNAME chains,
subdomains and DNS rebinding.

An agent that can pick its own address, or resolve a name itself, can reach
anything a name-based rule was written to prevent. Matching on the string the
agent supplied would be security theatre.

## Decision

Nothing is decided about a destination the runtime's own resolver did not
produce.

- **Resolve, record, then decide.** `Ledger::record` stores the chain of names
  followed and the addresses they produced. `Policy::evaluate` consults it. An
  address with no live resolution is denied (`UnresolvedAddress`); a name that
  was never resolved is denied (`UnresolvedName`). Direct IP access is
  therefore closed by construction rather than by a rule someone must remember
  to write.
- **A chain is only as allowed as its least allowed link.** Every name in a
  CNAME chain is judged, and the strictest answer wins. A denied domain hiding
  behind an allowed alias stays denied.
- **A resolution expires, and a re-answer replaces it.** TTLs are clamped to
  ten minutes, so a hostile server cannot pin an address indefinitely, and
  re-pointing a name drops the previous address rather than leaving it usable.
  That closes the rebinding window in both directions.
- **Addresses that are not globally routable are denied by default**, including
  169.254.169.254 — the cloud metadata service, which is what rebinding usually
  aims at. `allowing_private_addresses()` exists for runtimes whose purpose is
  an internal service, and says what it re-opens.
- **The most specific rule decides**, ties going to the stricter effect. This
  is what makes a closing `deny: "*"` a default rather than a
  prohibition that would swallow every allow above it.
- **Names have one spelling.** Lowercase ASCII, no trailing dot, punycode
  already applied, anything resembling an address refused as a name. A rule
  cannot be side-stepped by a different encoding of the same string.

## Consequences

- The resolver must be part of the runtime. An agent that resolves names
  itself and connects by address gets nothing through this layer, which is the
  intended outcome and also the reason enforcement needs the sandbox.
- Wildcard rules do not cover their own parent: `*.unknown.dev` says nothing
  about `unknown.dev`, which then falls to whatever else applies. This is
  narrower than some tools and is the fail-closed reading.
- Specificity ordering differs from `latch-policy`, where a deny always wins.
  That difference is deliberate — those rules arrive from a hierarchy of
  organizational levels — and it is a seam: if a level hierarchy is ever
  introduced for network rules, level must outrank specificity.
- The ledger is per session and unbounded until `forget_expired` is called. A
  long-running session needs the runtime to call it.
- Exfiltration detection is a heuristic and is documented as one at every call
  site. It recognizes a secret read followed by an unfamiliar upload, and a
  large volume leaving at once, and nothing more.

## Alternatives rejected

- **Match on the host string the agent supplied.** Zero cost, zero value: the
  agent chooses the string.
- **Resolve at decision time inside the policy engine.** Makes evaluation
  non-deterministic and network-bound — policy evaluation must be
  deterministic and stay within its latency budget —
  and invites a time-of-check/time-of-use gap between the policy lookup and the
  connection.
- **Allowlist addresses instead of names.** Correct for a fixed internal
  service, useless for anything behind a CDN whose addresses change hourly.
  Worth adding as an explicit address rule later; it is not a substitute.
