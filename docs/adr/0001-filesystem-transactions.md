# ADR 0001: Filesystem transactions stage into a private workspace

Status: accepted · Supersedes: none

## Context

Agent transactions are a core differentiator of Latch: an agent should be
able to work without changing the real state, and a human should be able to
commit or discard that work. Rollback integrity must be exact — verifiable
through hashes, metadata, permissions, directory structure and symlink targets.

A kernel overlay (overlayfs, a union mount, a microVM disk) gives this for free
and confines the process as well. It also requires privilege, differs on every
platform, and does not exist on Windows or macOS in the form we would need.

## Decision

Writes go through a mediated API into a private workspace outside the root.
Nothing touches the host until `commit`, and a commit journals the exact state
it replaces before replacing it.

- **Staging, not mounting.** `stage/` holds future content, `tomb/` holds
  deletions, `backup/` holds what a commit overwrote, `absent.log` records what
  did not exist. State lives in the filesystem, so a workspace can be reopened
  by another process — which is what makes `latch tx commit` possible.
- **Renames are derived, not recorded.** A move is staged as a deletion plus a
  creation, and reported as a rename when exactly one deletion and one creation
  share a content digest. Ambiguity is reported as what is certain rather than
  guessed at.
- **Paths are validated, never normalized.** One canonical spelling: relative,
  slash-separated, no traversal, no backslashes, no drive letters, no reserved
  device names, no trailing dots or spaces. Resolution walks component by
  component and refuses to traverse a symbolic link, because canonicalizing the
  leaf would follow exactly the link the check exists to catch.
- **Commit ends the transaction.** A partial commit applies the selection and
  closes it; unselected work stays in the workspace for inspection. A selection
  naming a path with no staged change is an error, not a quieter commit.
- **Rollback is idempotent.** Before a commit it discards staged work; after a
  commit it restores from the journal; after a failed restore it retries. A
  restore that cannot finish sets the state to `Inconsistent` and says which
  path defeated it, rather than reporting success.

## Consequences

- The guarantee only covers changes made *through* the API. This is mediation,
  not confinement, and the docs say so wherever the claim appears. Confinement
  arrives with `latch-sandbox`; until then the two must not be conflated.
- Content is copied twice on commit (into the journal, then into place), and a
  staged write costs a full copy. Acceptable for source trees, not for large
  binary assets; a content-addressed store would fix it if that changes.
- Timestamps, ownership and extended attributes are not restored. `Snapshot`
  therefore does not record them: a verification that ignores what it cannot
  restore would turn an honest gap into a false guarantee.
- Directory removal is refused rather than staged, because pruning directories
  that became empty cannot be done correctly without recording which of them
  the transaction itself created.
- Permission changes work on both permission models by mapping the write bit to
  the read-only flag where POSIX modes do not exist. Modes are only ever
  compared against modes from the same host.

## Alternatives rejected

- **overlayfs / union mounts.** Confines as well as stages, but needs privilege
  and is Linux-only. It remains the right answer inside `latch-sandbox`, where
  the privilege is already assumed; it is the wrong dependency for the layer
  every platform has to share.
- **Copy the whole tree, diff at commit.** Simple, and wrong at the size of a
  real repository: the cost is paid up front on every transaction, and it still
  cannot tell an agent's change from a concurrent one.
- **A content-addressed store with a manifest.** Better for large or repeated
  content, and a bigger format commitment than the first version of this layer
  should make. Worth revisiting when binary payloads arrive.
