use crate::prelude::*;
use libsip::Header;
use libsip::SipMessage;
use libsip::ResponseGenerator;

impl SipSessionProvider {
    pub(crate) async fn handle_bye<'a>(&mut self, msg: SipMessage, ctx: SessionCtx<'a>) -> NirahResult<()> {
        if let SipMessage::Request { headers, .. } = msg {
            if let Some(Header::CallId(call)) = headers.call_id() {
                if let Some(cseq) = headers.cseq() {
                    let bye_req = ResponseGenerator::new()
                       .code(200)
                       .header(cseq)
                       .header(Header::CallId(call.clone()))
                       .build()?;
                    let socket = unwrap_mut_or_else_not_connected!(self, socket, "Socket not connected");
                    let account = unwrap_or_else_not_connected!(self, acc, "Account not connected");
                    let data = format!("{}", bye_req);
                    socket.send_to(data.as_ref(), &account.get_socket_address()).await?;
                } else {
                    warn!("SIP Request does not contain a cseq header");
                }
                ctx.streaming.end_stream(streaming_ctx!(ctx), call).await?;
            } else {
                warn!("BYE request does not have a call-id header")
            }
        } else {
            warn!("handle_cancel given a SipMessage that was not a request.");
        }
        Ok(())
    }

    pub(crate) async fn bye<'a>(&mut self, _ctx: SessionCtx<'a>, req_call: String) -> NirahResult<()> {
        for call in &self.active {
            let call_id = call.call_id()?;
            if call_id == req_call {
                let bye_req = call.bye()?;
                let socket = unwrap_mut_or_else_not_connected!(self, socket, "Socket not connected");
                let account = unwrap_or_else_not_connected!(self, acc, "Account not connected");
                let data = format!("{}", bye_req);
                socket.send_to(data.as_ref(), &account.get_socket_address()).await?;
            }
        }
        Ok(())
    }
}
