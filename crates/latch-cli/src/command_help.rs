pub const HELP: &str = "\
latch - the control layer for autonomous agents

Usage:
  latch audit <segment-file>                   verify and print an audit segment
  latch tx begin <root> <workspace>             open a transaction over a directory
  latch tx changes <workspace>                  show staged changes
  latch tx commit <workspace> [path...]         apply all staged changes, or only these
  latch tx rollback <workspace>                 undo the transaction
  latch status <workspace>                      read-only state and retained-stage summary
  latch tx status <workspace>                   alias for status
  latch diff <workspace>                        alias for tx changes
  latch commit <workspace> [path...]            alias for tx commit
  latch rollback <workspace>                    alias for tx rollback
  latch policy validate <file>                  validate JSON (.json) or YAML (.yaml/.yml)
  latch policy explain <policy-file> <action> [resource]  offline hypothetical evaluation
  latch policy explain --help                   show synthetic evaluation context
  latch policy test <policy-file> <scenario-file>  offline JSON scenario tests
  latch policy test --help                      show the scenario fixture schema
  latch manifest check <file>                   validate a JSON manifest, not authorize it
  latch manifest diff <old> <new>                show requested permission changes, not grants
  latch capabilities                           show available workflows and limitations
  latch run [args...]                           unsupported until a confined backend exists
  latch --version                              print the package version
  latch --help

Policy and scenario reads are limited to 1 MiB; manifest reads to 4 MiB.
Audit segments are read up to 64 MiB and their hash chain is verified; the
printed chain head is what an external checkpoint is compared against.
File access and transactions use OS permissions, not a confinement backend.
Status compares retained staging against the current root, not commit history.
Policy explain is offline only, not runtime enforcement or audit history;
no event store exists. See policy explain --help for the synthetic context.
";

pub const CAPABILITIES: &str = "\
Available: audit verification, local filesystem transactions, policy validation,
transaction status, offline policy explanation and scenario tests,
JSON manifest validation and permission diff.
Policy tests use caller-supplied trusted fixtures, not authenticated identities.
Manifest checks do not grant permissions; diffs report expansions requiring review.
Unsupported: confined run, runtime enforcement, approval and budget fulfillment.
";
