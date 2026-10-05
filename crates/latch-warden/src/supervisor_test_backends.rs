use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt;
use std::rc::Rc;

// One globbable fixture surface for every split test module.
use latch_sandbox::SandboxBackend;
pub(crate) use latch_sandbox::{Operation, SandboxPlan, UnsupportedBackend};
pub(crate) use latch_substrate::{AssuranceLevel, BackendCapabilities, Control};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MockError;

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("mock failure")
    }
}

impl Error for MockError {}

pub(crate) type Calls = Rc<RefCell<Vec<(String, Operation)>>>;

pub(crate) struct MockBackend {
    name: String,
    calls: Calls,
    failure: Rc<Cell<Option<Operation>>>,
    allocated: Rc<Cell<bool>>,
}

#[derive(Clone)]
pub(crate) struct Probe {
    pub(crate) failure: Rc<Cell<Option<Operation>>>,
    pub(crate) allocated: Rc<Cell<bool>>,
}

impl MockBackend {
    pub(crate) fn new(name: &str, calls: &Calls) -> (Self, Probe) {
        let failure = Rc::new(Cell::new(None));
        let allocated = Rc::new(Cell::new(false));
        (
            Self {
                name: name.into(),
                calls: calls.clone(),
                failure: failure.clone(),
                allocated: allocated.clone(),
            },
            Probe { failure, allocated },
        )
    }

    fn perform(&self, operation: Operation) -> Result<(), MockError> {
        self.calls.borrow_mut().push((self.name.clone(), operation));
        if self.failure.get() == Some(operation) {
            Err(MockError)
        } else {
            Ok(())
        }
    }
}

impl SandboxBackend for MockBackend {
    type Error = MockError;

    fn capabilities(&self) -> BackendCapabilities {
        Control::ALL
            .into_iter()
            .fold(BackendCapabilities::default(), |caps, control| {
                caps.with_control(control, AssuranceLevel::CooperativeMediation)
            })
    }

    fn validate_plan(&self, _plan: &SandboxPlan) -> Result<(), Self::Error> {
        self.perform(Operation::Plan)
    }

    fn prepare_enforcement(&mut self, _plan: &SandboxPlan) -> Result<(), Self::Error> {
        self.allocated.set(true);
        self.perform(Operation::Prepare)
    }

    fn start_enforced(&mut self) -> Result<(), Self::Error> {
        assert!(self.allocated.get());
        self.perform(Operation::Start)
    }

    fn freeze_all(&mut self) -> Result<(), Self::Error> {
        self.perform(Operation::Freeze)
    }

    fn terminate_all_and_cleanup(&mut self) -> Result<(), Self::Error> {
        self.perform(Operation::Terminate)?;
        self.allocated.set(false);
        Ok(())
    }
}

pub(crate) use crate::supervisor_test_fixtures::{authorization, limits, register, requirements};
