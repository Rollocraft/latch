use crate::bounded_file_read::bounded_read;
use latch_policy::{MAX_POLICY_DOCUMENT_BYTES, PolicyDocument};
use std::{ffi::OsString, path::Path};

pub(crate) fn load_policy(path: &OsString) -> Result<PolicyDocument, String> {
    let bytes = bounded_read(path, MAX_POLICY_DOCUMENT_BYTES)?;
    let document = match Path::new(path).extension().and_then(|ext| ext.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("json") => PolicyDocument::from_json(&bytes),
        Some(ext) if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") => {
            PolicyDocument::from_yaml(&bytes)
        }
        _ => return Err("policy file must have .json, .yaml or .yml extension".into()),
    };
    document.map_err(|e| format!("invalid policy {path:?}: {e:?}"))
}
