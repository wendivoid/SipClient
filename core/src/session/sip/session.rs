use async_trait::async_trait;
use libsip::core::Method;
use libsip::parse_message;

use crate::prelude::*;
use super::errors::*;

use std::time::Duration;
use std::time::Instant;

#[async_trait]
impl SessionProvider for SipSessionProvider {

    fn account_id(&self) -> NirahResult<u32> {
        if let Some(acc) = &self.acc {
            Ok(acc.id.clone())
        } else {
            Err(NirahError::SessionNotAssociatedWithAccount)
        }
    }

    async fn read_future<'a>(&mut self) -> NirahResult<Vec<u8>> {
        if let Some(socket) = &mut self.socket {
            let mut buf = vec![ 0u8 ; 65535 ];
            let amt = socket.recv(&mut buf).await?;
            Ok(buf[..amt].to_vec())
        } else {
            not_connected("Tried to read before listening socket has yet to be connected")
        }
    }

    async fn timeout_time(&self) -> NirahResult<Option<Instant>> {
        if let Some(client) = &self.client {
            if let Some(timeout) = self.reg_timeout {
                Ok(Some(timeout + Duration::from_secs(client.registry().expires() as u64)))
            } else {
                warn!("Sip Session timout isn't set.");
                Ok(None)
            }
        } else {
            not_connected("Tried to use sip client before socket was connected")
        }
    }

    async fn handle_event<'a>(&mut self, ctx: SessionCtx<'a>, event: SessionEvent) ->NirahResult<()> {
        match event {
            SessionEvent::Timeout => {
                trace!("Running sip timeout handler.");
                self.register().await?;
                Ok(())
            },
            SessionEvent::Data { data } => {
                let (_, sip_message) = parse_message(&data)?;
                if let &libsip::SipMessage::Request { method, .. } = &sip_message {
                    match method {
                        Method::Message => self.handle_message(sip_message, ctx).await?,
                        Method::Invite => self.handle_invite(sip_message, ctx).await?,
                        Method::Bye => self.handle_bye(sip_message, ctx).await?,
                        Method::Cancel => self.handle_cancel(sip_message, ctx).await?,
                        _ => {}
                    }
                } else {
                    warn!("SessionProvider::handle_event received SipMessage that was not a request.");
                }
                Ok(())
            },
            SessionEvent::AcceptInvite { invite } => self.accept_invite(ctx, invite).await,
            SessionEvent::Bye { call } => self.bye(ctx, call).await,
            SessionEvent::Transaction { transaction } => {
                match &transaction.data {
                    TransactionEventData::TextMessage { message } => {
                        let socket = unwrap_mut_or_else_not_connected!(self, socket, "Socket not connected");
                        let client = unwrap_mut_or_else_not_connected!(self, client, "Client not connected");
                        let account = unwrap_or_else_not_connected!(self, acc, "Account not connected");
                        let contact = ctx.contacts.get_contact(transaction.contact).await?.unwrap();
                        let msg = client.write_message(message.as_bytes().to_vec(), contact.uri)?;
                        let msg_data = format!("{}", msg);
                        let addr = account.get_socket_address();
                        socket.send_to(msg_data.as_ref(), &addr).await?;
                    },
                    TransactionEventData::Invitation { } => {

                    }
                }
                Ok(())
            }
        }
    }
}
