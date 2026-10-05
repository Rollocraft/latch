use super::*;

pub(super) fn request() -> Request {
    Request {
        version: VERSION,
        request_id: "r1".into(),
        idempotency_id: Some("i1".into()),
        caller: CallerMetadata {
            tenant: "tenant".into(),
            actor: "actor".into(),
            session: "session".into(),
        },
        action: RequestedAction {
            name: "file.read".into(),
            arguments: vec!["".into(), "a b".into(), "é\n".into()],
            resource: "workspace/file".into(),
            environment: "dev".into(),
        },
    }
}
