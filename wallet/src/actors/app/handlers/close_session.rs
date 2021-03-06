use actix::prelude::*;

use crate::actors::App;
use crate::api;

impl Message for api::CloseSessionRequest {
    type Result = Result<api::CloseSessionResponse, api::Error>;
}

impl Handler<api::CloseSessionRequest> for App {
    type Result = Result<api::CloseSessionResponse, api::Error>;

    fn handle(&mut self, msg: api::CloseSessionRequest, _ctx: &mut Self::Context) -> Self::Result {
        self.close_session(msg.session_id);
        Ok(())
    }
}
