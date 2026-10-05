use crate::AssuranceLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Plan,
    Prepare,
    Start,
    Freeze,
    Terminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxState {
    Unplanned,
    Planned,
    Prepared,
    Running,
    Frozen,
    Terminated,
    Failed { operation: Operation },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementStatus {
    NotEstablished,
    BackendReported { assurance: AssuranceLevel },
    Unknown,
}
