use dropshot::{HttpError, HttpResponseOk, Path, RequestContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::body::CompressedTypedBody;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub(crate) struct MachineId {
    pub machine_id: String,
}

#[dropshot::api_description]
pub(crate) trait SantaSyncServerApi {
    type Context;

    #[endpoint(
        method = POST,
        path = "/preflight/{machine_id}",
        content_type = "application/json",
    )]
    async fn preflight_post(
        rqctx: RequestContext<Self::Context>,
        path_params: Path<MachineId>,
        body_params: CompressedTypedBody<santa_types::PreflightOptions>,
    ) -> Result<HttpResponseOk<santa_types::Preflight>, HttpError>;

    #[endpoint(
        method = POST,
        path = "/eventupload/{machine_id}",
        content_type = "application/json",
    )]
    async fn eventupload_post(
        rqctx: RequestContext<Self::Context>,
        path_params: Path<MachineId>,
        body_params: CompressedTypedBody<santa_types::EventUploadOptions>,
    ) -> Result<HttpResponseOk<santa_types::Empty>, HttpError>;

    #[endpoint(
        method = POST,
        path = "/ruledownload/{machine_id}",
        content_type = "application/json",
    )]
    async fn ruledownload_post(
        rqctx: RequestContext<Self::Context>,
        path_params: Path<MachineId>,
    ) -> Result<HttpResponseOk<santa_types::Rules>, HttpError>;

    #[endpoint(
        method = POST,
        path = "/postflight/{machine_id}",
        content_type = "application/json",
    )]
    async fn postflight_post(
        _rqctx: RequestContext<Self::Context>,
        _path_params: Path<MachineId>,
        _body_params: CompressedTypedBody<santa_types::PostflightOptions>,
    ) -> Result<HttpResponseOk<santa_types::Empty>, HttpError>;
}
