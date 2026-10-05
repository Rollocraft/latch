use crate::{AssuranceLevel, Control};
use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityGap {
    pub control: Control,
    pub required: AssuranceLevel,
    pub advertised: Option<AssuranceLevel>,
}

impl fmt::Display for CapabilityGap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} requires {:?}; backend advertises {:?}",
            self.control, self.required, self.advertised
        )
    }
}

impl Error for CapabilityGap {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedRequirements {
    pub(crate) gaps: Vec<CapabilityGap>,
}

impl UnsupportedRequirements {
    pub fn gaps(&self) -> &[CapabilityGap] {
        &self.gaps
    }
}

impl fmt::Display for UnsupportedRequirements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unsupported enforcement requirements")?;
        for gap in &self.gaps {
            write!(f, "; {gap}")?;
        }
        Ok(())
    }
}

impl Error for UnsupportedRequirements {}
