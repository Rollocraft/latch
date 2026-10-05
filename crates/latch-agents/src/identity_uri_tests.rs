use crate::manifest_test_fixtures::*;
use crate::*;

#[test]
fn identity_binding_rejects_invalid_and_cross_tenant_ids() {
    for id in [
        "agent://other/coder",
        "agent://acme",
        "agent://acme/../coder",
        "agent://acme//coder",
        "agent://acme/%61",
        "agent://acme/coder?x",
    ] {
        let mut runtime = identity();
        runtime.id = id.into();
        assert!(IdentityBinding::from_identity(&runtime).is_err());
    }
    let binding = IdentityBinding::from_identity(&identity()).unwrap();
    assert_eq!(binding.id(), identity().id);
    assert_eq!(binding.organization(), "acme");
    assert_eq!(binding.team(), "backend");
    assert_eq!(binding.owner(), "operator");
    assert_eq!(binding.purpose(), "coding");
    assert_eq!(binding.runtime(), "latch");
    assert_eq!(binding.environment(), "development");
    assert_eq!(binding.device(), "workstation");
}
