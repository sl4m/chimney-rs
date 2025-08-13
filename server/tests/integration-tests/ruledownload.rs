use dropshot::test_util::read_json;

use crate::test_util::{
    ContentEncoding, DEFAULT_CONFIG_PATH, EventLogMode, MachineId, TestContext, build_request,
};

const PREFIX_URI: &str = "/ruledownload";

fn build_uri(machine_id: &str) -> String {
    format!("{PREFIX_URI}/{machine_id}")
}

#[tokio::test]
async fn ruledownload_deflate_specified_machine_id() {
    let machine_id = MachineId::One.to_string();
    let testctx = TestContext::new(
        "ruledownload_deflate_specified_machine_id",
        DEFAULT_CONFIG_PATH,
        EventLogMode::None,
    );
    let request_body = r#"{}"#;
    let uri = testctx.inner.client_testctx.url(&build_uri(&machine_id));
    let request = build_request(request_body, &ContentEncoding::Deflate, uri);

    let mut response = testctx
        .inner
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");
    let rules = read_json::<santa_types::Rules>(&mut response).await;
    let client_config = testctx.config_for(&machine_id);

    assert_eq!(client_config.rules, rules.rules);

    testctx.teardown().await;
}

#[tokio::test]
async fn ruledownload_gzip_specified_machine_id() {
    let machine_id = MachineId::Two.to_string();
    let testctx = TestContext::new(
        "ruledownload_gzip_specified_machine_id",
        DEFAULT_CONFIG_PATH,
        EventLogMode::None,
    );
    let request_body = r#"{}"#;
    let uri = testctx.inner.client_testctx.url(&build_uri(&machine_id));
    let request = build_request(request_body, &ContentEncoding::Gzip, uri);

    let mut response = testctx
        .inner
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");
    let rules = read_json::<santa_types::Rules>(&mut response).await;
    let client_config = testctx.config_for(&machine_id);

    assert_eq!(client_config.rules, rules.rules);

    testctx.teardown().await;
}
