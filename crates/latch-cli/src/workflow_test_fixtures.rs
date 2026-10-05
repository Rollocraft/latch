use crate::{run, scenarios};
use serde_json::{Value, json};
use std::{ffi::OsString, path::PathBuf};

pub(crate) struct Files(pub(crate) PathBuf);

impl Files {
    pub(crate) fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "latch-cli-workflow-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    pub(crate) fn write(&self, name: &str, bytes: impl AsRef<[u8]>) -> OsString {
        let path = self.0.join(name);
        std::fs::write(&path, bytes).unwrap();
        path.into_os_string()
    }
}

impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub(crate) fn call(args: &[&std::ffi::OsStr]) -> (Result<(), String>, String) {
    let mut output = Vec::new();
    let result = run(
        args.iter().map(|arg| arg.to_os_string()).collect(),
        &mut output,
    );
    (result, String::from_utf8(output).unwrap())
}

pub(crate) fn policy_value(effect: Value) -> Value {
    json!({"version":1,"policies":[{"id":"default","level":"company","rules":[{
        "id":"rule","description":"fixture rule","action":{"kind":"any"},
        "resource":{"kind":"any"},"environment":{"kind":"any"},
        "actor":{"kind":"any"},"effect":effect
    }]}]})
}

pub(crate) fn scenario_value() -> Value {
    let start = scenarios::HELP.find("{\"version\"").unwrap();
    let end = scenarios::HELP.find("\nExpected outcomes:").unwrap();
    serde_json::from_str(&scenarios::HELP[start..end]).unwrap()
}

pub(crate) fn manifest_value() -> Value {
    json!({
        "schema_version":1,
        "agent":{"name":"coder","version":"1"},
        "identity":{
            "id":"agent://acme/coder","organization":"acme","team":"dev",
            "owner":"operator","purpose":"testing","runtime":"latch",
            "environment":"development","device":"fixture"
        },
        "requires":[{
            "action":"file.read", "environment":"development",
            "resource":{"kind":"exact","resource":"file:///project/a"}
        }]
    })
}
