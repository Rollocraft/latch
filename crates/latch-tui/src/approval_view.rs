use crate::{ApprovalResponse, Rendered, ResponseError};

/// A rendered approval review plus whether allow-once may be offered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalView {
    pub(crate) rendered: Rendered,
    pub(crate) allow_once_available: bool,
}

impl ApprovalView {
    pub fn rendered(&self) -> &Rendered {
        &self.rendered
    }

    pub fn allow_once_available(&self) -> bool {
        self.allow_once_available
    }

    pub fn parse_response(&self, input: &str) -> Result<ApprovalResponse, ResponseError> {
        let response = input.parse()?;
        if response == ApprovalResponse::AllowOnce && !self.allow_once_available {
            return Err(ResponseError::AllowUnavailable);
        }
        Ok(response)
    }
}
