use crate::prelude::*;

impl SipSessionProvider {
    pub(crate) async fn handle_message<'a>(&mut self, msg: nirah_sip::SipMessage, ctx: SessionCtx<'a>) -> NirahResult<()> {
        debug!("Received Sip Text Message: {:?}", msg);
        if let nirah_sip::SipMessage::Request { uri, headers, body, .. } = msg {
            let helper = nirah_sip::client::MessageHelper::new(uri, headers, body)?;
            let response = helper.received()?;
            let response_data = format!("{}", response);
            let socket = unwrap_mut_or_else_not_connected!(self, socket, "Socket not connected");
            let account = unwrap_or_else_not_connected!(self, acc, "Account not connected");
            let acc = { unwrap_or_else_not_connected!(self, acc, "Local Uri has not been set").clone() };
            let addr = acc.get_socket_address();
            socket.send_to(response_data.as_ref(), &addr).await?;
            let from_header = helper.from()?;
            if let Some(contact) = ctx.contacts.get_contact_from_uri(from_header.uri.clone()).await? {
                let logged_message = NewTransactionEvent {
                    account: account.id,
                    contact: contact.id,
                    sent: false,
                    time: chrono::Utc::now().naive_utc(),
                    data: NewTransactionEventData::NewTextMessage {
                        message: String::from_utf8(helper.data())?
                    }
                };
                let log_id = ctx.database.log(contact.id, logged_message).await?;
                let msg = ctx.database.get_log(contact.id, log_id).await?.unwrap();
                let params = NotifierParams {
                    invite_id: None,
                };
                let arg = NotifierArgument { account: &account, contact: &contact, params, msg };
                ctx.notifier.new_transaction(arg).await?;
            } else {
                debug!("Creating new contact for: {:?}, ", from_header.uri);
                let id = ctx.contacts.create_contact(NewContact { display_name: from_header.display_name, uri: from_header.uri }).await?;
                let logged_message = NewTransactionEvent {
                    account: account.id,
                    contact: id,
                    sent: false,
                    time: chrono::Utc::now().naive_utc(),
                    data: NewTransactionEventData::NewTextMessage {
                        message: String::from_utf8(helper.data())?
                    }
                };
                let log_id = ctx.database.log(id, logged_message).await?;
                let msg = ctx.database.get_log(id, log_id).await?.unwrap();
                let contact = ctx.contacts.get_contact(id).await?.unwrap();
                let params = NotifierParams {
                    invite_id: None,
                };
                let arg = NotifierArgument { account: &account, contact: &contact, params, msg: &msg };
                ctx.notifier.new_transaction(arg).await?;
            }
        } else {
            warn!("handle_message received a sip message that was not a request");
        }
        Ok(())
    }
}
