use crate::validate_policies::validate_policies;
use crate::{Policy, PolicyError};

#[derive(Debug, Clone)]
pub struct PolicyEngine {
    pub(crate) policies: Vec<Policy>,
}

impl PolicyEngine {
    pub fn new(mut policies: Vec<Policy>) -> Result<Self, PolicyError> {
        validate_policies(&policies)?;
        policies.sort_by(|a, b| (a.level, &a.id).cmp(&(b.level, &b.id)));
        Ok(Self { policies })
    }
}
