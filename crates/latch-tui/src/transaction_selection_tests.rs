use crate::presentation_test_fixtures::*;
use crate::transaction_selection::render_changes;
use crate::*;
use latch_core::Digest;
use latch_transaction::{Change, ChangeSet, RelativePath};

#[test]
fn transaction_review_is_metadata_only_and_selection_is_explicit() {
    let changes = [
        Change::Created {
            path: RelativePath::new("new.txt").unwrap(),
            digest: Digest::from_bytes([0; 32]),
        },
        Change::Renamed {
            from: RelativePath::new("old.txt").unwrap(),
            to: RelativePath::new("renamed.txt").unwrap(),
            digest: Digest::from_bytes([0; 32]),
        },
    ];
    let rendered = render_changes(&changes, &[1]);
    assert!(rendered.text.contains("[ ] 0: CREATED"));
    assert!(rendered.text.contains("[x] 1: RENAMED"));
    assert!(rendered.text.contains("From: old.txt"));
    assert!(rendered.text.contains("Path: renamed.txt"));
    assert!(!rendered.text.contains(&"0".repeat(64)));
    assert!(!render_changes(&changes, &[]).text.contains("[x]"));
    let invalid = render_changes(&changes, &[0, 99]);
    assert!(invalid.text.contains("INVALID SELECTION"));
    assert!(!invalid.text.contains("[x]"));
    assert!(
        render_transaction_selection(&ChangeSet::default(), &[])
            .text
            .contains("No changes")
    );
    assert_safe(&rendered.text);
}
