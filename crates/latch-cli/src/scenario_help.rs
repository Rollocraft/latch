pub const HELP: &str = "\
Usage: latch policy test <policy-file> <scenario-file>
Scenarios are JSON only, up to 1 MiB, with unknown fields rejected.
Schema: {\"version\":1,\"scenarios\":[{\"id\":\"example\",\"now\":10,
  \"session\":{\"id\":\"s1\",\"policies\":[\"default\"],\"identity\":{
    \"id\":\"agent://acme/coder\",\"organization\":\"acme\",\"team\":\"dev\",
    \"owner\":\"operator\",\"purpose\":\"testing\",\"provider\":\"local\",
    \"model\":\"fixture\",\"model_version\":\"1\",\"runtime\":\"latch\",
    \"environment\":\"development\",\"device\":\"fixture\",
    \"created_at\":0,\"expires_at\":100,\"trust_level\":0}},
  \"action\":{\"id\":\"a1\",\"name\":\"file.read\",\"resource\":\"file:///project/a\",
    \"arguments\":[],\"risk\":0,\"reversibility\":\"fully_reversible\"},
  \"expected\":\"allow\"}]}
Expected outcomes: allow, deny, approval_required, limited.
Reversibility: fully_reversible, partially_reversible, irreversible.
All fields are required. At most 1024 scenarios; IDs must be unique and nonempty.
Sessions are active at now; action attribution is derived from the session.
Identity, session and action are validated before testing. Fixtures are supplied
by the caller for offline simulation only, not authentication or authorization.
Only outcomes are asserted; mismatches return failure. Nothing is executed.
";
