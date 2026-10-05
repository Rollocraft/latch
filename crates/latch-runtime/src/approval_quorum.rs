use super::ApprovalError;
use std::collections::BTreeSet;

/// Distinct authenticated principals required for actions that policy marks ASK.
/// This is trusted runtime configuration, not agent-supplied authorization.
/// The caller still authenticates each principal before submitting an approval.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalQuorum {
    approvers: BTreeSet<String>,
    required: usize,
}

impl ApprovalQuorum {
    pub fn new(
        approvers: impl IntoIterator<Item = String>,
        required: usize,
    ) -> Result<Self, ApprovalError> {
        let approvers: BTreeSet<_> = approvers.into_iter().collect();
        if approvers.iter().any(|name| {
            name.is_empty() || name.trim() != name || name.chars().any(char::is_control)
        }) {
            return Err(ApprovalError::InvalidApprover);
        }
        if required == 0 || required > approvers.len() {
            return Err(ApprovalError::InvalidQuorum);
        }
        Ok(Self {
            approvers,
            required,
        })
    }

    pub fn required(&self) -> usize {
        self.required
    }

    pub fn permits(&self, approver: &str) -> bool {
        self.approvers.contains(approver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quorum_rejects_unreachable_or_unsafe_configurations() {
        for (approvers, required) in [
            (vec![], 1),
            (vec!["solo".to_string()], 0),
            (vec!["solo".to_string()], 2),
            (vec!["a".into(), " ".into()], 1),
            (vec!["pad".into(), " pad".into()], 1),
            (vec!["ok".into(), "ctl\u{7}".into()], 1),
        ] {
            assert!(
                ApprovalQuorum::new(approvers, required).is_err(),
                "quorum must reject unsafe configuration {required}"
            );
        }
    }

    #[test]
    fn duplicate_names_collapse_and_membership_is_exact() {
        let quorum =
            ApprovalQuorum::new(["alice".to_string(), "bob".into(), "alice".into()], 2).unwrap();
        assert_eq!(quorum.required(), 2);
        assert!(quorum.permits("alice"));
        assert!(quorum.permits("bob"));
        assert!(!quorum.permits("Alice"), "membership is case exact");
        assert!(!quorum.permits("mallory"));
    }
}
