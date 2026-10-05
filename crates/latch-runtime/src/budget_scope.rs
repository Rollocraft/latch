#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BudgetScope {
    organization: String,
    session: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetError {
    InvalidScope,
    InvalidBudget,
    MissingCost(String),
    UnexpectedCost(String),
    Exhausted(String),
    Unavailable,
}

impl BudgetScope {
    pub fn new(organization: String, session: String) -> Result<Self, BudgetError> {
        if [&organization, &session]
            .iter()
            .any(|s| s.trim().is_empty() || s.chars().any(char::is_control))
        {
            return Err(BudgetError::InvalidScope);
        }
        Ok(Self {
            organization,
            session,
        })
    }
}
