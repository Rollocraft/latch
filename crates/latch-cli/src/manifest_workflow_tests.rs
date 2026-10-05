use crate::workflow_test_fixtures::*;
use serde_json::json;

#[test]
fn manifests_check_and_diff_requested_permissions() {
    let files = Files::new();
    let old = files.write("old.json", manifest_value().to_string());
    let (result, output) = call(&["manifest".as_ref(), "check".as_ref(), &old]);
    assert!(result.is_ok(), "{result:?}");
    assert!(output.contains("not authorization"));
    let (result, output) = call(&["manifest".as_ref(), "diff".as_ref(), &old, &old]);
    assert!(result.is_ok(), "{result:?}");
    assert!(output.contains("requires_review: false"));
    let mut new = manifest_value();
    new["requires"][0]["resource"] = json!({"kind":"subtree","resource":"file:///project"});
    let new = files.write("new.json", new.to_string());
    let (result, output) = call(&["manifest".as_ref(), "diff".as_ref(), &old, &new]);
    assert!(result.is_ok(), "{result:?}");
    assert!(output.contains("requires_review: true"));
    assert!(output.contains("\"added\": [\n"));
    assert!(output.contains("\"removed\": [\n"));
    assert!(output.contains("\"expanded\": [\n"));
    let (result, output) = call(&["manifest".as_ref(), "diff".as_ref(), &new, &old]);
    assert!(result.is_ok(), "{result:?}");
    assert!(output.contains("requires_review: false"));
}
