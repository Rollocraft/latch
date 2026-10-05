use super::*;
use std::{
    cell::{Cell, RefCell},
    error::Error,
    fmt,
    rc::Rc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MockError;

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("mock backend failure")
    }
}

impl Error for MockError {}

#[derive(Clone)]
pub(super) struct MockBackend {
    pub(super) caps: Rc<Cell<BackendCapabilities>>,
    pub(super) calls: Rc<RefCell<Vec<Operation>>>,
    pub(super) failure: Rc<Cell<Option<Operation>>>,
    pub(super) reject_limits: Rc<Cell<bool>>,
    pub(super) prepared: Option<SandboxPlan>,
}

impl MockBackend {
    pub(super) fn new(assurance: AssuranceLevel) -> Self {
        let caps = Control::ALL
            .into_iter()
            .fold(BackendCapabilities::default(), |caps, control| {
                caps.with_control(control, assurance)
            });
        Self {
            caps: Rc::new(Cell::new(caps)),
            calls: Rc::new(RefCell::new(Vec::new())),
            failure: Rc::new(Cell::new(None)),
            reject_limits: Rc::new(Cell::new(false)),
            prepared: None,
        }
    }

    pub(super) fn perform(&self, operation: Operation) -> Result<(), MockError> {
        self.calls.borrow_mut().push(operation);
        if self.failure.get() == Some(operation) {
            Err(MockError)
        } else {
            Ok(())
        }
    }
}
