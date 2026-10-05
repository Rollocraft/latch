// Existing manifest assertions grouped by validation or permission concern.
#[path = "action_name_tests.rs"]
mod action_name_tests;
#[path = "bound_request_tests.rs"]
mod bound_request_tests;
#[path = "capability_environment_tests.rs"]
mod capability_environment_tests;
#[path = "capability_limit_tests.rs"]
mod capability_limit_tests;
#[path = "identity_attribution_tests.rs"]
mod identity_attribution_tests;
#[path = "identity_uri_tests.rs"]
mod identity_uri_tests;
#[path = "manifest_json_tests.rs"]
mod manifest_json_tests;
#[path = "manifest_roundtrip_tests.rs"]
mod manifest_roundtrip_tests;
#[path = "manifest_schema_tests.rs"]
mod manifest_schema_tests;
#[path = "maximum_capabilities_tests.rs"]
mod maximum_capabilities_tests;
#[path = "metadata_validation_tests.rs"]
mod metadata_validation_tests;
#[path = "model_authority_tests.rs"]
mod model_authority_tests;
#[path = "permission_diff_tests.rs"]
mod permission_diff_tests;
#[path = "permission_expansion_tests.rs"]
mod permission_expansion_tests;
#[path = "permission_identity_tests.rs"]
mod permission_identity_tests;
#[path = "permission_subset_tests.rs"]
mod permission_subset_tests;
#[path = "permission_union_tests.rs"]
mod permission_union_tests;
#[path = "permission_version_tests.rs"]
mod permission_version_tests;
#[path = "request_authority_tests.rs"]
mod request_authority_tests;
#[path = "resource_scope_tests.rs"]
mod resource_scope_tests;
#[path = "resource_uri_tests.rs"]
mod resource_uri_tests;
#[path = "unknown_manifest_fields_tests.rs"]
mod unknown_manifest_fields_tests;
