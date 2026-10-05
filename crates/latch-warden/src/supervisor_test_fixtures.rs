use crate::supervisor_test_backends::{AssuranceLevel, Calls, MockBackend, Probe};
use crate::{
    EnforcementRequirements, ExternalAuthorization, ResourceLimits, SessionHandle, Supervisor,
};
use std::time::Duration;

pub(crate) fn authorization(organization: &str) -> ExternalAuthorization {
    ExternalAuthorization::from_trusted_control_plane(organization).unwrap()
}

pub(crate) fn limits() -> ResourceLimits {
    ResourceLimits::new(
        Duration::from_secs(1),
        1024,
        1,
        1024,
        Duration::from_secs(2),
    )
    .unwrap()
}

pub(crate) fn requirements() -> EnforcementRequirements {
    EnforcementRequirements::new(AssuranceLevel::CooperativeMediation)
}

pub(crate) fn register(
    supervisor: &mut Supervisor<MockBackend>,
    authorization: &ExternalAuthorization,
    session_id: &str,
    calls: &Calls,
) -> (SessionHandle, Probe) {
    let (backend, probe) = MockBackend::new(session_id, calls);
    let handle = supervisor
        .register(authorization, session_id, backend, limits(), requirements())
        .unwrap();
    (handle, probe)
}
