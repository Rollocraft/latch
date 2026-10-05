use crate::{ManifestError, error::invalid};

pub(crate) fn validate_resource(value: &str) -> Result<(), ManifestError> {
    let error = || {
        invalid(
            "resource",
            "expected a canonical hierarchical resource URI without wildcards, encoding, query or fragment",
        )
    };
    if value.len() > 2048 {
        return Err(error());
    }
    let (scheme, rest) = value.split_once("://").ok_or_else(error)?;
    if scheme.is_empty()
        || !scheme.as_bytes()[0].is_ascii_lowercase()
        || !scheme
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"+-.".contains(&b))
    {
        return Err(error());
    }
    let (authority, path) = rest
        .split_once('/')
        .map_or((rest, None), |(a, p)| (a, Some(p)));
    if authority.is_empty() {
        if scheme != "file" || path.is_none() {
            return Err(error());
        }
    } else if authority == "."
        || authority == ".."
        || authority.ends_with('.')
        || !authority
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"-_.".contains(&b))
    {
        return Err(error());
    }
    if let Some(path) = path {
        for segment in path.split('/') {
            if segment.is_empty()
                || segment == "."
                || segment == ".."
                || segment.ends_with('.')
                || !segment
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
            {
                return Err(error());
            }
        }
    }
    Ok(())
}
