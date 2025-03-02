use dropshot::test_util::read_json;

use crate::test_util::{
    build_request, ContentEncoding, EventLogMode, MachineId, TestContext, DEFAULT_CONFIG_PATH,
};

const PREFIX_URI: &str = "/preflight";

fn build_uri(machine_id: &str) -> String {
    format!("{}/{}", PREFIX_URI, machine_id)
}

#[tokio::test]
async fn preflight_deflate_specified_machine_id() {
    let machine_id = MachineId::One.to_string();
    let testctx = TestContext::new(
        "preflight_deflate_specified_machine_id",
        DEFAULT_CONFIG_PATH,
        EventLogMode::None,
    );
    let request_body = r#"{
        "serial_num": "serial_num",
        "hostname": "hostname",
        "os_version": "os_version",
        "os_build": "os_build",
        "santa_version": "santa_version",
        "primary_user": "primary_user",
        "client_mode": "MONITOR"
    }"#;
    let uri = testctx.inner.client_testctx.url(&build_uri(&machine_id));
    let request = build_request(request_body, &ContentEncoding::Deflate, uri);

    let mut response = testctx
        .inner
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");
    let preflight = read_json::<santa_types::Preflight>(&mut response).await;
    let client_config = testctx.config_for(&machine_id);

    assert_eq!(client_config.preflight, preflight);

    testctx.teardown().await;
}

#[tokio::test]
async fn preflight_gzip_specified_machine_id() {
    let machine_id = MachineId::Two.to_string();
    let testctx = TestContext::new(
        "preflight_gzip_specified_machine_id",
        DEFAULT_CONFIG_PATH,
        EventLogMode::None,
    );
    let request_body = r#"{
        "serial_num": "serial_num",
        "hostname": "hostname",
        "os_version": "os_version",
        "os_build": "os_build",
        "santa_version": "santa_version",
        "primary_user": "primary_user",
        "client_mode": "MONITOR"
    }"#;
    let uri = testctx.inner.client_testctx.url(&build_uri(&machine_id));
    let request = build_request(request_body, &ContentEncoding::Gzip, uri);

    let mut response = testctx
        .inner
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");
    let preflight = read_json::<santa_types::Preflight>(&mut response).await;
    let client_config = testctx.config_for(&machine_id);

    assert_eq!(client_config.preflight, preflight);

    testctx.teardown().await;
}
