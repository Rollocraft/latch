use crate::manifest_test_fixtures::*;
use crate::*;

#[test]
fn binding_checks_all_attribution_fields_and_validity() {
    for field in 0..8 {
        let mut runtime = identity();
        match field {
            0 => runtime.id = "agent://acme/other".into(),
            1 => {
                runtime.organization = "other".into();
                runtime.id = "agent://other/coder".into();
            }
            2 => runtime.team = "other".into(),
            3 => runtime.owner = "other".into(),
            4 => runtime.purpose = "other".into(),
            5 => runtime.runtime = "other".into(),
            6 => runtime.environment = "production".into(),
            _ => runtime.device = "other".into(),
        }
        assert_eq!(
            sample().bind(&runtime, 20),
            Err(ManifestError::IdentityMismatch)
        );
    }
    for now in [0, 9, 100, u64::MAX] {
        assert!(sample().bind(&identity(), now).is_err());
    }
    for now in [10, 99] {
        assert!(sample().bind(&identity(), now).is_ok());
    }
    let mut runtime = identity();
    runtime.trust_level = 101;
    assert!(sample().bind(&runtime, 20).is_err());
}
