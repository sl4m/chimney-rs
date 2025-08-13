use std::time::Duration;

use crate::test_util::{
    ContentEncoding, DEFAULT_CONFIG_PATH, EventLogMode, MachineId, TestContext, build_request,
};

const PREFIX_URI: &str = "/eventupload";
const DEFAULT_REQUEST_BODY: &str = r#"{
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
const DEFAULT_EXPECTED_RESULTS: &str = r#"
{"machine_id":"machine-id-1234","file_sha256":"file_sha256","file_path":"file_path","file_name":"file_name","executing_user":"executing_user","execution_time":123412354345.4,"loggedin_users":"abcd, defg","current_sessions":"1, 2","decision":"ALLOW_BINARY","file_bundle_id":"file_bundle_id","file_bundle_path":"file_bundle_path","file_bundle_executable_rel_path":"file_bundle_executable_rel_path","file_bundle_name":"file_bundle_name","file_bundle_version":"file_bundle_version","file_bundle_version_string":"file_bundle_version_string","file_bundle_hash":"file_bundle_hash","file_bundle_hash_millis":12345,"file_bundle_binary_count":56,"pid":1234,"ppid":5678,"parent_name":"parent_name","quarantine_data_url":"quarantine_data_url","quarantine_referer_url":"quarantine_referer_url","quarantine_agent_bundle_id":"quarantine_agent_bundle_id","signing_chain.0.sha256":"sha256","signing_chain.0.cn":"cn","signing_chain.0.org":"org","signing_chain.0.ou":"ou","signing_chain.0.valid_from":12345,"signing_chain.0.valid_until":57689,"signing_id":"signing_id","team_id":"team_id","cdhash":"cdhash"}
"#;

fn build_uri(machine_id: &str) -> String {
    format!("{PREFIX_URI}/{machine_id}")
}

#[tokio::test]
async fn eventupload_deflate_specified_machine_id() {
    let machine_id = MachineId::One.to_string();
    let testctx = TestContext::new(
        "eventupload_deflate_specified_machine_id",
        DEFAULT_CONFIG_PATH,
        EventLogMode::Persist,
    );
    let request_body = DEFAULT_REQUEST_BODY;
    serde_json::from_str::<santa_types::EventUploadOptions>(request_body)
        .expect("json is parseable");
    let uri = testctx.inner.client_testctx.url(&build_uri(&machine_id));
    let request = build_request(request_body, &ContentEncoding::Deflate, uri);

    testctx
        .inner
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");

    tokio::time::sleep(Duration::from_millis(50)).await;
    let logged_events = testctx.events_as_string();
    assert_eq!(1, logged_events.len());
    let expected_results = DEFAULT_EXPECTED_RESULTS.trim();
    assert_eq!(expected_results, logged_events.first().unwrap());

    testctx.teardown().await;
}

#[tokio::test]
async fn eventupload_gzip_specified_machine_id() {
    let machine_id = MachineId::One.to_string();
    let testctx = TestContext::new(
        "eventupload_deflate_specified_machine_id",
        DEFAULT_CONFIG_PATH,
        EventLogMode::Persist,
    );
    let request_body = DEFAULT_REQUEST_BODY;
    serde_json::from_str::<santa_types::EventUploadOptions>(request_body)
        .expect("json is parseable");
    let uri = testctx.inner.client_testctx.url(&build_uri(&machine_id));
    let request = build_request(request_body, &ContentEncoding::Gzip, uri);

    testctx
        .inner
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");

    tokio::time::sleep(Duration::from_millis(50)).await;
    let logged_events = testctx.events_as_string();
    assert_eq!(1, logged_events.len());
    let expected_results = DEFAULT_EXPECTED_RESULTS.trim();
    assert_eq!(expected_results, logged_events.first().unwrap());

    testctx.teardown().await;
}

#[tokio::test]
async fn eventupload_no_persistence() {
    let machine_id = MachineId::One.to_string();
    let testctx = TestContext::new(
        "eventupload_deflate_specified_machine_id",
        DEFAULT_CONFIG_PATH,
        EventLogMode::None,
    );
    let request_body = DEFAULT_REQUEST_BODY;
    serde_json::from_str::<santa_types::EventUploadOptions>(request_body)
        .expect("json is parseable");
    let uri = testctx.inner.client_testctx.url(&build_uri(&machine_id));
    let request = build_request(request_body, &ContentEncoding::Gzip, uri);

    testctx
        .inner
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");

    let logged_events = testctx.events_as_string();
    assert_eq!(0, logged_events.len());

    testctx.teardown().await;
}

#[tokio::test]
async fn eventupload_deflate_multiple_events() {
    let machine_id = MachineId::One.to_string();
    let testctx = TestContext::new(
        "eventupload_deflate_specified_machine_id",
        DEFAULT_CONFIG_PATH,
        EventLogMode::Persist,
    );
    let request_body: &str = r#"{
        "events": [
            {
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
            },
            {
                "file_sha256": "file_sha256_2",
                "file_path": "file_path_2",
                "file_name": "file_name_2",
                "executing_user": "executing_user_2",
                "execution_time": 123412354345.5,
                "loggedin_users": ["abcd_2", "defg_2"],
                "current_sessions": ["1_2", "2_2"],
                "decision": "ALLOW_CERTIFICATE",
                "file_bundle_id": "file_bundle_id_2",
                "file_bundle_path": "file_bundle_path_2",
                "file_bundle_executable_rel_path": "file_bundle_executable_rel_path_2",
                "file_bundle_name": "file_bundle_name_2",
                "file_bundle_version": "file_bundle_version_2",
                "file_bundle_version_string": "file_bundle_version_string_2",
                "file_bundle_hash": "file_bundle_hash_2",
                "file_bundle_hash_millis": 12346,
                "file_bundle_binary_count": 57,
                "pid": 4321,
                "ppid": 8765,
                "parent_name": "parent_name_2",
                "quarantine_data_url": "quarantine_data_url_2",
                "quarantine_referer_url": "quarantine_referer_url_2",
                "quarantine_timestamp": "quarantine_timestamp_2",
                "quarantine_agent_bundle_id": "quarantine_agent_bundle_id_2",
                "signing_chain": [{
                    "sha256": "sha256_2",
                    "cn": "cn_2",
                    "org": "org_2",
                    "ou": "ou_2",
                    "valid_from": 54321,
                    "valid_until": 98765
                }],
                "signing_id": "signing_id_2",
                "team_id": "team_id_2",
                "cdhash": "cdhash_2"
            }
        ]
    }"#;
    serde_json::from_str::<santa_types::EventUploadOptions>(request_body)
        .expect("json is parseable");
    let uri = testctx.inner.client_testctx.url(&build_uri(&machine_id));
    let request = build_request(request_body, &ContentEncoding::Deflate, uri);

    testctx
        .inner
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");

    tokio::time::sleep(Duration::from_millis(50)).await;
    let logged_events = testctx.events_as_string();
    assert_eq!(2, logged_events.len());
    let expected_results = DEFAULT_EXPECTED_RESULTS.trim();
    assert_eq!(expected_results, logged_events.first().unwrap());

    testctx.teardown().await;
}
