#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Developer,
    TeamLead,
    SecurityAdmin,
    OrganizationAdmin,
    Auditor,
    Approver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    ReadPolicy,
    ReadSession,
    ReadAudit,
    ManagePolicy,
    ApproveAction,
    ManageIdentity,
    KillSession,
}

impl Role {
    pub fn permits(self, operation: Operation) -> bool {
        match operation {
            Operation::ReadPolicy | Operation::ReadSession => true,
            Operation::ReadAudit => matches!(
                self,
                Self::TeamLead | Self::SecurityAdmin | Self::OrganizationAdmin | Self::Auditor
            ),
            Operation::ManagePolicy | Operation::KillSession => matches!(
                self,
                Self::TeamLead | Self::SecurityAdmin | Self::OrganizationAdmin
            ),
            Operation::ApproveAction => {
                matches!(
                    self,
                    Self::TeamLead | Self::OrganizationAdmin | Self::Approver
                )
            }
            Operation::ManageIdentity => self == Self::OrganizationAdmin,
        }
    }
}
