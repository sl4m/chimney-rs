use dropshot::{HttpError, HttpResponseOk, Path, RequestContext};
use slog::{info, o};

use crate::api::{MachineId, SantaSyncServerApi};
use crate::body::CompressedTypedBody;
use crate::{Context, SantaSyncServerApiImpl};
use santa_types::Empty;

impl SantaSyncServerApi for SantaSyncServerApiImpl {
    type Context = Context;

    async fn preflight_post(
        rqctx: RequestContext<Self::Context>,
        path_params: Path<MachineId>,
        _body_params: CompressedTypedBody<santa_types::PreflightOptions>,
    ) -> Result<HttpResponseOk<santa_types::Preflight>, HttpError> {
        let machine_id = path_params.into_inner().machine_id;
        let client_config = rqctx.context().store.config_for(&machine_id);
        Ok(HttpResponseOk(client_config.preflight))
    }

    async fn eventupload_post(
        rqctx: RequestContext<Self::Context>,
        path_params: Path<MachineId>,
        body_params: CompressedTypedBody<santa_types::EventUploadOptions>,
    ) -> Result<HttpResponseOk<Empty>, HttpError> {
        if let Some(log) = &rqctx.context().event_log {
            let machine_id = path_params.into_inner().machine_id;
            let log = log.new(o!("machine_id" => machine_id));
            let event_upload_options = body_params.into_inner();
            for event in event_upload_options.events.iter() {
                info!(log, ""; &event);
            }
        }
        Ok(HttpResponseOk(Empty {}))
    }

    async fn ruledownload_post(
        rqctx: RequestContext<Self::Context>,
        path_params: Path<MachineId>,
    ) -> Result<HttpResponseOk<santa_types::Rules>, HttpError> {
        let machine_id = path_params.into_inner().machine_id;
        let client_config = rqctx.context().store.config_for(&machine_id);
        let rules = client_config.rules;
        Ok(HttpResponseOk(santa_types::Rules { rules }))
    }

    async fn postflight_post(
        _rqctx: RequestContext<Self::Context>,
        _path_params: Path<MachineId>,
        _body_params: CompressedTypedBody<santa_types::PostflightOptions>,
    ) -> Result<HttpResponseOk<Empty>, HttpError> {
        Ok(HttpResponseOk(Empty {}))
    }
}
