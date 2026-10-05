use crate::{
    IdentityBinding, ManifestError,
    error::{invalid, validate_label, validate_text},
};

impl IdentityBinding {
    pub(crate) fn validate(&self) -> Result<(), ManifestError> {
        validate_text("identity.id", &self.id)?;
        validate_label("identity.organization", &self.organization)?;
        validate_label("identity.environment", &self.environment)?;
        for (field, value) in [
            ("identity.team", &self.team),
            ("identity.owner", &self.owner),
            ("identity.purpose", &self.purpose),
            ("identity.runtime", &self.runtime),
            ("identity.device", &self.device),
        ] {
            validate_text(field, value)?;
        }
        let path = self
            .id
            .strip_prefix("agent://")
            .ok_or_else(|| invalid("identity.id", "expected agent URI"))?;
        if path.split('/').count() < 2
            || path.split('/').next() != Some(self.organization.as_str())
            || path.split('/').any(|p| {
                p.is_empty()
                    || p == "."
                    || p == ".."
                    || !p
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
            })
        {
            return Err(invalid(
                "identity.id",
                "invalid or cross-organization agent URI",
            ));
        }
        Ok(())
    }
}
