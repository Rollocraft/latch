# Latch

**The control layer for autonomous software.**

Latch is a vendor-neutral security and runtime layer for AI agents.

It sits between autonomous agents and the systems they interact with, controlling what they are allowed to access, change, execute, or communicate with.

The long-term goal is to provide a common infrastructure layer for:

- agent identity
- permissions and policies
- sandboxed execution
- filesystem and process isolation
- transactional changes
- commit / rollback
- network control
- approvals
- audit logging
- privileged enforcement

## Core idea

Agents should not receive unrestricted access to the host system.

Instead, Latch aims to make agent actions:

- controlled
- attributable
- reviewable
- reversible where possible
- enforceable outside the agent itself