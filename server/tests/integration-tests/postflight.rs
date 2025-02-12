use crate::test_util::{
    build_request, ContentEncoding, EventLogMode, MachineId, TestContext, DEFAULT_CONFIG_PATH,
};

const PREFIX_URI: &str = "/v1/postflight";

fn build_uri(machine_id: &str) -> String {
    format!("{}/{}", PREFIX_URI, machine_id)
}

#[tokio::test]
async fn postflight_success() {
    let testctx = TestContext::new(
        "postflight_success",
        DEFAULT_CONFIG_PATH,
        EventLogMode::None,
    );
    let request_body = r#"{
        "rules_received": 200,
        "rules_processed": 100
    }"#;
    let uri = testctx
        .inner
        .client_testctx
        .url(&build_uri(&MachineId::One.to_string()));
    let request = build_request(request_body, &ContentEncoding::Deflate, uri);

    testctx
        .inner
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");

    testctx.teardown().await;
}

#[tokio::test]
async fn postflight_optional_fields() {
    let testctx = TestContext::new(
        "postflight_optional_fields",
        DEFAULT_CONFIG_PATH,
        EventLogMode::None,
    );
    let request_body = r#"{}"#;
    let uri = testctx
        .inner
        .client_testctx
        .url(&build_uri(&MachineId::One.to_string()));
    let request = build_request(request_body, &ContentEncoding::Gzip, uri);

    testctx
        .inner
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");

    testctx.teardown().await;
}
