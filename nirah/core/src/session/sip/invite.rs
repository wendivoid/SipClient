use crate::prelude::*;

impl SipSessionProvider {
    pub(crate) async fn handle_invite<'a>(&mut self, msg: nirah_sip::SipMessage, ctx: SessionCtx<'a>) -> NirahResult<()> {
        if let nirah_sip::SipMessage::Request { uri, headers, body, .. } = msg {
            let helper = nirah_sip::client::InviteHelper::new(uri.clone(), headers, body)?;
            let socket = unwrap_mut_or_else_not_connected!(self, socket, "Socket not connected");
            let account = unwrap_or_else_not_connected!(self, acc, "Account not connected");
            let ring_req = helper.ringing()?;
            let data = format!("{}", ring_req);
            socket.send_to(data.as_ref(), &account.get_socket_address()).await?;
            ctx.audio.start_ringing(ctx.config).await?;
            if let Some(contact) = ctx.contacts.get_contact_from_uri(uri).await? {
                let new_msg = NewTransactionEvent {
                    account: account.id,
                    contact: contact.id,
                    sent: false,
                    time: chrono::Utc::now().naive_utc(),
                    data: NewTransactionEventData::NewInvitation {

                    }
                };
                let log_id = ctx.database.log(contact.id, new_msg).await?;
                let logged_message = ctx.database.get_log(contact.id, log_id).await?.unwrap();
                let invite_id = self.invitations.len();
                self.invitations.push(helper);
                let params = NotifierParams {
                    invite_id: Some(invite_id),
                };
                let arg = NotifierArgument { account: &account, contact: &contact, params, msg: &logged_message };
                ctx.notifier.new_transaction(arg).await?;
            } else {
                let from_header = helper.from()?;
                debug!("Creating new contact for: {:?}, ", from_header.uri);
                let id = ctx.contacts.create_contact(NewContact { display_name: from_header.display_name, uri: from_header.uri }).await?;
                let logged_message = NewTransactionEvent {
                    account: account.id,
                    contact: id,
                    sent: false,
                    time: chrono::Utc::now().naive_utc(),
                    data: NewTransactionEventData::NewInvitation {

                    }
                };
                let log_id = ctx.database.log(id, logged_message).await?;
                let msg = ctx.database.get_log(id, log_id).await?.unwrap();
                let contact = ctx.contacts.get_contact(id).await?.unwrap();
                let invite_id = self.invitations.len();
                self.invitations.push(helper);
                let params = NotifierParams {
                    invite_id: Some(invite_id),
                };
                let arg = NotifierArgument { account: &account, contact: &contact, params, msg: &msg };
                ctx.notifier.new_transaction(arg).await?;
            }
        } else {
            warn!("handle_invite passed a sip message that was not a request.")
        }
        Ok(())
    }

    pub async fn accept_invite<'a>(&mut self, ctx: SessionCtx<'a>, invite: usize) -> NirahResult<()> {
        if let Some(_invitation) = self.invitations.get(invite) {
            ctx.streaming.handle_session(streaming_ctx!(ctx)).await?;
        } else {
            warn!("Attempted to accept a non existant invite");
        }
        Ok(())
    }
}
