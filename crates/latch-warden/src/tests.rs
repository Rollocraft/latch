// Existing supervisor assertions grouped by lifecycle and organization scoping.
#[path = "organization_kill_tests.rs"]
mod organization_kill_tests;
#[path = "registration_validation_tests.rs"]
mod registration_validation_tests;
#[path = "supervisor_failure_tests.rs"]
mod supervisor_failure_tests;
#[path = "supervisor_tests.rs"]
mod supervisor_tests;
#[path = "tenant_scoping_tests.rs"]
mod tenant_scoping_tests;
