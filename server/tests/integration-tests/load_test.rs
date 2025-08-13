#[allow(unused_imports)]
use std::time::Duration;

#[allow(unused_imports)]
use crate::test_util::{
    ContentEncoding, DEFAULT_CONFIG_PATH, EventLogMode, MachineId, TestContext, build_request,
};

#[allow(dead_code)]
const DEFAULT_EVENTUPLOAD_BODY: &str = r#"{
    "events": [{
        "file_sha256": "file_sha256",
        "file_path": "file_path",
        "file_name": "file_name",
        "executing_user": "executing_user",
        "execution_time": 123412354345.4,
        "loggedin_users": ["abcd", "defg"],
        "current_sessions": ["1", "2"],
        "decision": "ALLOW_BINARY",
        "file_bundle_id": "file_bundle_id",
        "file_bundle_path": "file_bundle_path",
        "file_bundle_executable_rel_path": "file_bundle_executable_rel_path",
        "file_bundle_name": "file_bundle_name",
        "file_bundle_version": "file_bundle_version",
        "file_bundle_version_string": "file_bundle_version_string",
        "file_bundle_hash": "file_bundle_hash",
        "file_bundle_hash_millis": 12345,
        "file_bundle_binary_count": 56,
        "pid": 1234,
        "ppid": 5678,
        "parent_name": "parent_name",
        "quarantine_data_url": "quarantine_data_url",
        "quarantine_referer_url": "quarantine_referer_url",
        "quarantine_timestamp": "quarantine_timestamp",
        "quarantine_agent_bundle_id": "quarantine_agent_bundle_id",
        "signing_chain": [{
            "sha256": "sha256",
            "cn": "cn",
            "org": "org",
            "ou": "ou",
            "valid_from": 12345,
            "valid_until": 57689
        }],
        "signing_id": "signing_id",
        "team_id": "team_id",
        "cdhash": "cdhash"
    }]
}"#;

#[allow(dead_code)]
fn build_uri(machine_id: &str) -> String {
    format!("/eventupload/{machine_id}")
}

#[tokio::test]
#[cfg(feature = "load_tests")]
async fn eventupload_load() {
    let mut handles = vec![];
    let testctx = TestContext::new(
        "eventupload_load",
        DEFAULT_CONFIG_PATH,
        EventLogMode::Persist,
    );
    let inner = &testctx.inner;
    let request_body = DEFAULT_EVENTUPLOAD_BODY;
    serde_json::from_str::<santa_types::EventUploadOptions>(request_body)
        .expect("json is parseable");

    for _ in 0..1000 {
        let client_testctx = inner.client_testctx.clone();
        let handle = tokio::spawn(async move {
            let machine_id = MachineId::One.to_string();
            let uri = client_testctx.url(&build_uri(&machine_id));
            let request = build_request(request_body.into(), &ContentEncoding::Deflate, uri);
            client_testctx
                .make_request_with_request(request, http::StatusCode::OK)
                .await
                .expect("expected success");
        });
        handles.push(handle);
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    for handle in handles {
        handle.await.unwrap();
    }
    let logged_events = testctx.events_as_string();
    assert_eq!(1000, logged_events.len());

    testctx.teardown().await;
}
