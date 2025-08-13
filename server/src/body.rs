use std::io::Read;

use async_trait::async_trait;
use bytes::Bytes;
use dropshot::{
    ApiEndpointBodyContentType, Body, ClientErrorStatusCode, ExclusiveExtractor, ExtractorMetadata,
    HttpError, RequestContext, ServerContext, TypedBody, UntypedBody,
};
use flate2::read::{MultiGzDecoder, ZlibDecoder};
use hyper::Request;
use schemars::JsonSchema;
use serde::de::DeserializeOwned;

const CONTENT_ENCODING_DEFLATE: &str = "deflate";
const CONTENT_ENCODING_GZIP: &str = "gzip";
const CONTENT_ENCODING_NONE: &str = "none";

#[derive(Debug)]
pub struct CompressedTypedBody<BodyType: JsonSchema + DeserializeOwned + Send + Sync> {
    inner: BodyType,
}

impl<BodyType: JsonSchema + DeserializeOwned + Send + Sync> CompressedTypedBody<BodyType> {
    pub fn into_inner(self) -> BodyType {
        self.inner
    }
}

#[async_trait]
impl<BodyType> ExclusiveExtractor for CompressedTypedBody<BodyType>
where
    BodyType: JsonSchema + DeserializeOwned + Send + Sync + 'static,
{
    async fn from_request<Context: ServerContext>(
        rqctx: &RequestContext<Context>,
        request: Request<Body>,
    ) -> Result<CompressedTypedBody<BodyType>, HttpError> {
        let (parts, body) = request.into_parts();
        let request = Request::from_parts(parts.clone(), body);
        let untyped_body = UntypedBody::from_request(rqctx, request).await?;

        let content_encoding = parts
            .headers
            .get(http::header::CONTENT_ENCODING)
            .map(|value| {
                value.to_str().map_err(|e| {
                    HttpError::for_bad_request(None, format!("invalid content-encoding: {e}"))
                })
            })
            .unwrap_or(Ok(CONTENT_ENCODING_NONE))?;

        let decoded_body = decode_body(content_encoding, untyped_body.as_bytes())?;
        let request = Request::from_parts(parts, Body::with_content(decoded_body));
        let typed_body = TypedBody::<BodyType>::from_request(rqctx, request).await?;

        Ok(Self {
            inner: typed_body.into_inner(),
        })
    }

    fn metadata(content_type: ApiEndpointBodyContentType) -> ExtractorMetadata {
        TypedBody::<BodyType>::metadata(content_type)
    }
}

/// Santa client sends zlib-wrapped request data despite using Content-Encoding: deflate
fn decode_body(content_encoding: &str, body: &[u8]) -> Result<Bytes, HttpError> {
    match content_encoding {
        CONTENT_ENCODING_DEFLATE => {
            let mut zlib = ZlibDecoder::new(body);
            let mut decoded = vec![];
            zlib.read_to_end(&mut decoded).map_err(|e| {
                HttpError::for_client_error(
                    None,
                    ClientErrorStatusCode::UNSUPPORTED_MEDIA_TYPE,
                    format!("not a zlib request body: {e}"),
                )
            })?;
            Ok(Bytes::from(decoded))
        }
        CONTENT_ENCODING_GZIP => {
            let mut gz = MultiGzDecoder::new(body);
            let mut decoded = vec![];
            gz.read_to_end(&mut decoded).map_err(|e| {
                HttpError::for_client_error(
                    None,
                    ClientErrorStatusCode::UNSUPPORTED_MEDIA_TYPE,
                    format!("not a gzip request body: {e}"),
                )
            })?;
            Ok(Bytes::from(decoded))
        }
        CONTENT_ENCODING_NONE => Ok(Bytes::copy_from_slice(body)),
        _ => Err(HttpError::for_client_error(
            None,
            ClientErrorStatusCode::UNSUPPORTED_MEDIA_TYPE,
            format!("unsupported content-encoding \"{content_encoding}\""),
        )),
    }
}
