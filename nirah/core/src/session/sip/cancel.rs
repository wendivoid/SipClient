use crate::prelude::*;
use nirah_sip::Header;
use nirah_sip::SipMessage;

impl SipSessionProvider {
    pub(crate) async fn handle_cancel<'a>(&mut self, msg: SipMessage, ctx: SessionCtx<'a>) -> NirahResult<()> {
        if let SipMessage::Request { headers, .. } = msg {
            if let Some(Header::CSeq(count, _)) = headers.cseq() {
                let mut dex = None;
                for (index, invitation) in self.invitations.iter().enumerate() {
                    if invitation.check_cseq(count)? {
                        dex = Some(index);
                    }
                }
                if let Some(index) = dex {
                    self.invitations.remove(index);
                    if self.invitations.len() == 0 {
                        ctx.audio.stop_ringing(ctx.config).await?;
                        let socket = unwrap_mut_or_else_not_connected!(self, socket, "Socket not connected");
                        let account = unwrap_or_else_not_connected!(self, acc, "Account not connected");
                        let client = unwrap_mut_or_else_not_connected!(self, client, "Client not connected");
                        let (msg_response1, msg_response2) = client.cancel_response(&headers)?;
                        let msg_data1 = format!("{}", msg_response1);
                        let msg_data2 = format!("{}", msg_response2);
                        let addr = account.get_socket_address();
                        socket.send_to(msg_data1.as_ref(), &addr).await?;
                        socket.send_to(msg_data2.as_ref(), &addr).await?;

                    }
                } else {
                    warn!("No matching invitation for cancel request.")
                }
            }

        } else {
            warn!("handle_cancel given a SipMessage that was not a request.");
        }

        Ok(())
    }
}
