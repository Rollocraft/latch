use crate::manifest_test_fixtures::*;
use latch_core::{Reversibility, RiskScore};

#[test]
fn request_lookup_never_uses_risk_reversibility_or_arguments_as_authority() {
    let bound = sample().bind(&identity(), 20).unwrap();
    let mut request = action();
    request.risk = RiskScore::new(0).unwrap();
    request.reversibility = Reversibility::FullyReversible;
    request.arguments = vec!["uninterpreted".into()];
    assert!(bound.requests_action(&session(), &request, 20).unwrap());
    request.name = "file.write".into();
    assert!(!bound.requests_action(&session(), &request, 20).unwrap());
    request.name = "file.read".into();
    request.resource = "file:///other".into();
    assert!(!bound.requests_action(&session(), &request, 20).unwrap());
}
