use crate::{AssuranceLevel, CapabilityGap, Control};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BackendCapabilities {
    controls: [Option<AssuranceLevel>; 10],
}

impl BackendCapabilities {
    pub fn with_control(mut self, control: Control, assurance: AssuranceLevel) -> Self {
        self.controls[control.index()] = Some(assurance);
        self
    }

    pub fn assurance(&self, control: Control) -> Option<AssuranceLevel> {
        self.controls[control.index()]
    }

    pub fn check(&self, control: Control, required: AssuranceLevel) -> Result<(), CapabilityGap> {
        let advertised = self.assurance(control);
        if advertised.is_some_and(|level| level.satisfies(required)) {
            Ok(())
        } else {
            Err(CapabilityGap {
                control,
                required,
                advertised,
            })
        }
    }
}
