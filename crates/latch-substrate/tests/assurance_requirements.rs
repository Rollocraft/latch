use latch_substrate::{AssuranceLevel, BackendCapabilities, Control, EnforcementRequirements};

fn capabilities(assurance: AssuranceLevel) -> BackendCapabilities {
    Control::ALL
        .into_iter()
        .fold(BackendCapabilities::default(), |caps, control| {
            caps.with_control(control, assurance)
        })
}

#[test]
fn default_requirements_fail_closed() {
    let requirements = EnforcementRequirements::default();
    assert_eq!(requirements.assurance(), AssuranceLevel::OsConfinement);
    let error = requirements
        .validate(&BackendCapabilities::default())
        .unwrap_err();
    assert_eq!(error.gaps().len(), Control::ALL.len());
    assert!(error.gaps().iter().all(|gap| gap.advertised.is_none()));
}

#[test]
fn every_control_is_required_individually() {
    for missing in Control::ALL {
        let caps = Control::ALL
            .into_iter()
            .filter(|&control| control != missing)
            .fold(BackendCapabilities::default(), |caps, control| {
                caps.with_control(control, AssuranceLevel::OsConfinement)
            });
        let error = EnforcementRequirements::protected()
            .validate(&caps)
            .unwrap_err();
        assert_eq!(error.gaps().len(), 1);
        assert_eq!(error.gaps()[0].control, missing);
    }
}

#[test]
fn cooperative_mediation_cannot_satisfy_confinement() {
    let caps = capabilities(AssuranceLevel::CooperativeMediation);
    assert!(
        EnforcementRequirements::new(AssuranceLevel::CooperativeMediation)
            .validate(&caps)
            .is_ok()
    );
    let error = EnforcementRequirements::protected()
        .validate(&caps)
        .unwrap_err();
    assert_eq!(error.gaps().len(), Control::ALL.len());
    assert!(error.gaps().iter().all(|gap| {
        gap.advertised == Some(AssuranceLevel::CooperativeMediation)
            && gap.required == AssuranceLevel::OsConfinement
    }));
}

#[test]
fn mixed_assurance_does_not_upgrade_weaker_control() {
    for weak in Control::ALL {
        let caps = capabilities(AssuranceLevel::OsConfinement)
            .with_control(weak, AssuranceLevel::CooperativeMediation);
        let error = EnforcementRequirements::protected()
            .validate(&caps)
            .unwrap_err();
        assert_eq!(error.gaps().len(), 1);
        assert_eq!(error.gaps()[0].control, weak);
    }
}

#[test]
fn confinement_capabilities_satisfy_both_levels() {
    let caps = capabilities(AssuranceLevel::OsConfinement);
    for assurance in [
        AssuranceLevel::CooperativeMediation,
        AssuranceLevel::OsConfinement,
    ] {
        assert!(
            EnforcementRequirements::new(assurance)
                .validate(&caps)
                .is_ok()
        );
    }
}
