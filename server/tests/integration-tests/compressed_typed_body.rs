use bytes::Bytes;
use dropshot::test_util::{TestContext, read_json};
use dropshot::{
    ApiDescription, Body, ConfigDropshot, HandlerTaskMode, HttpError, RequestContext, endpoint,
};
use http::header::{CONTENT_ENCODING, CONTENT_TYPE};
use http::{Method, Response, StatusCode};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use slog::o;

use crate::test_util::{create_log_context, gzip_encode, zlib_encode};
use chimney_server::body::CompressedTypedBody;

fn api() -> ApiDescription<usize> {
    let mut api = ApiDescription::new();
    api.register(api_compressed_typed_body).unwrap();
    api
}

#[endpoint {
    method = POST,
    path = "/echo",
}]
async fn api_compressed_typed_body(
    _rqctx: RequestContext<usize>,
    body: CompressedTypedBody<TestRequest>,
) -> Result<Response<Body>, HttpError> {
    let serialized = serde_json::to_string(&body.into_inner()).unwrap();
    let body = Bytes::copy_from_slice(serialized.as_bytes());
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::with_content(body))?;

    Ok(response)
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, JsonSchema)]
struct TestRequest {
    field1: bool,
    field2: u32,
    field3: String,
}

#[tokio::test]
async fn decodes_deflate_request() {
    let api = api();
    let testctx = setup_test("decodes_deflate_request", api);

    let uri = testctx.client_testctx.url("/echo");
    let test_req = TestRequest {
        field1: true,
        field2: 20,
        field3: "test".into(),
    };
    let encoded = zlib_encode(&test_req);
    let request = hyper::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_ENCODING, "deflate")
        .body(Body::with_content(encoded))
        .expect("invalid request");

    let mut response = testctx
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");
    let body = read_json::<TestRequest>(&mut response).await;

    assert_eq!(test_req, body);

    testctx.teardown().await;
}

#[tokio::test]
async fn decodes_gzip_request() {
    let api = api();
    let testctx = setup_test("decodes_gzip_request", api);

    let uri = testctx.client_testctx.url("/echo");
    let test_req = TestRequest {
        field1: true,
        field2: 20,
        field3: "test".into(),
    };
    let encoded = gzip_encode(&test_req);
    let request = hyper::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_ENCODING, "gzip")
        .body(Body::with_content(encoded))
        .expect("invalid request");

    let mut response = testctx
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");
    let body = read_json::<TestRequest>(&mut response).await;

    assert_eq!(test_req, body);

    testctx.teardown().await;
}

#[tokio::test]
async fn uncompressed_request() {
    let api = api();
    let testctx = setup_test("uncompressed_request", api);

    let uri = testctx.client_testctx.url("/echo");
    let test_req = TestRequest {
        field1: true,
        field2: 20,
        field3: "test".into(),
    };
    let serialized = serde_json::to_string(&test_req).unwrap();
    let body = Bytes::copy_from_slice(serialized.as_bytes());
    let request = hyper::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::with_content(body))
        .expect("invalid request");

    let mut response = testctx
        .client_testctx
        .make_request_with_request(request, http::StatusCode::OK)
        .await
        .expect("expected success");
    let body = read_json::<TestRequest>(&mut response).await;

    assert_eq!(test_req, body);

    testctx.teardown().await;
}

#[tokio::test]
async fn unsupported_content_encoding() {
    let api = api();
    let testctx = setup_test("unsupported_content_encoding", api);

    let uri = testctx.client_testctx.url("/echo");
    let test_req = TestRequest {
        field1: true,
        field2: 20,
        field3: "test".into(),
    };
    let serialized = serde_json::to_string(&test_req).unwrap();
    let body = Bytes::copy_from_slice(serialized.as_bytes());
    let request = hyper::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_ENCODING, "bad")
        .body(Body::with_content(body))
        .expect("invalid request");

    let response = testctx
        .client_testctx
        .make_request_with_request(request, http::StatusCode::UNSUPPORTED_MEDIA_TYPE)
        .await;
    let err = response.unwrap_err();
    assert_eq!(err.message, "unsupported content-encoding \"bad\"");

    testctx.teardown().await;
}

#[tokio::test]
async fn invalid_content_encoding() {
    let api = api();
    let testctx = setup_test("invalid_content_encoding", api);

    let uri = testctx.client_testctx.url("/echo");
    let test_req = TestRequest {
        field1: true,
        field2: 20,
        field3: "test".into(),
    };
    let serialized = serde_json::to_string(&test_req).unwrap();
    let body = Bytes::copy_from_slice(serialized.as_bytes());
    let request = hyper::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_ENCODING, vec![0xFF, 0xFF])
        .body(Body::with_content(body))
        .expect("invalid request");

    let response = testctx
        .client_testctx
        .make_request_with_request(request, http::StatusCode::BAD_REQUEST)
        .await;
    let err = response.unwrap_err();
    assert_eq!(
        err.message,
        "invalid content-encoding: failed to convert header to a str"
    );

    testctx.teardown().await;
}

#[tokio::test]
async fn invalid_deflate_request() {
    let api = api();
    let testctx = setup_test("invalid_deflate_request", api);

    let uri = testctx.client_testctx.url("/echo");
    let test_req = TestRequest {
        field1: true,
        field2: 20,
        field3: "test".into(),
    };
    let serialized = serde_json::to_string(&test_req).unwrap();
    let body = Bytes::copy_from_slice(serialized.as_bytes());
    let request = hyper::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_ENCODING, "deflate")
        .body(Body::with_content(body))
        .expect("invalid request");

    let response = testctx
        .client_testctx
        .make_request_with_request(request, http::StatusCode::UNSUPPORTED_MEDIA_TYPE)
        .await;
    let err = response.unwrap_err();
    assert_eq!(
        err.message,
        "not a zlib request body: corrupt deflate stream"
    );

    testctx.teardown().await;
}

#[tokio::test]
async fn invalid_gzip_request() {
    let api = api();
    let testctx = setup_test("invalid_gzip_request", api);

    let uri = testctx.client_testctx.url("/echo");
    let test_req = TestRequest {
        field1: true,
        field2: 20,
        field3: "test".into(),
    };
    let serialized = serde_json::to_string(&test_req).unwrap();
    let body = Bytes::copy_from_slice(serialized.as_bytes());
    let request = hyper::Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_ENCODING, "gzip")
        .body(Body::with_content(body))
        .expect("invalid request");

    let response = testctx
        .client_testctx
        .make_request_with_request(request, http::StatusCode::UNSUPPORTED_MEDIA_TYPE)
        .await;
    let err = response.unwrap_err();
    assert_eq!(err.message, "not a gzip request body: invalid gzip header");

    testctx.teardown().await;
}

fn setup_test(test_name: &str, api: ApiDescription<usize>) -> TestContext<usize> {
    let default_handler_task_mode = HandlerTaskMode::Detached;
    let config_dropshot: ConfigDropshot = ConfigDropshot {
        default_handler_task_mode,
        ..Default::default()
    };
    let logctx = create_log_context(test_name);
    let log = logctx.log.new(o!());
    TestContext::new(api, 0_usize, &config_dropshot, Some(logctx), log)
}
