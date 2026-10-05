use crate::{AssuranceLevel, BackendCapabilities, Control, UnsupportedRequirements};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnforcementRequirements {
    assurance: AssuranceLevel,
}

impl EnforcementRequirements {
    pub fn new(assurance: AssuranceLevel) -> Self {
        Self { assurance }
    }

    pub fn protected() -> Self {
        Self::new(AssuranceLevel::OsConfinement)
    }

    pub fn assurance(self) -> AssuranceLevel {
        self.assurance
    }

    pub fn required_controls(self) -> &'static [Control] {
        &Control::ALL
    }

    pub fn validate(
        self,
        capabilities: &BackendCapabilities,
    ) -> Result<(), UnsupportedRequirements> {
        let gaps: Vec<_> = self
            .required_controls()
            .iter()
            .filter_map(|&control| capabilities.check(control, self.assurance).err())
            .collect();
        if gaps.is_empty() {
            Ok(())
        } else {
            Err(UnsupportedRequirements { gaps })
        }
    }
}

impl Default for EnforcementRequirements {
    fn default() -> Self {
        Self::protected()
    }
}
