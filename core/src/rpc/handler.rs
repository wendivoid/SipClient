use async_trait::async_trait;

use crate::prelude::*;

pub struct DefaultRpcHandler;

impl DefaultRpcHandler {

    pub fn new() -> DefaultRpcHandler {
        DefaultRpcHandler
    }
}

#[async_trait]
impl Provider for DefaultRpcHandler {

    fn nirah_provider_identifier(&self) -> &'static str {
        "DefaultHandler"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[async_trait]
impl RpcHandlerProvider for DefaultRpcHandler {
    async fn handle<'a>(&mut self, req: RpcRequest, mut ctx: ServerCtx<'a>) -> NirahResult<RpcResponse> {
        match req {
            RpcRequest::AboutNirah => {
                let sip_session = SipSessionProvider::new();
                let sessions = vec![
                    (sip_session.nirah_provider_identifier().into(), sip_session.nirah_provider_version().into()),
                ];
                Ok(RpcResponse::AboutNirah {
                    accounts: (ctx.accounts.nirah_provider_identifier().into(), ctx.accounts.nirah_provider_version().into()),
                    config: (ctx.config.nirah_provider_identifier().into(), ctx.config.nirah_provider_version().into()),
                    contacts: (ctx.contacts.nirah_provider_identifier().into(), ctx.contacts.nirah_provider_version().into()),
                    database: (ctx.database.nirah_provider_identifier().into(), ctx.database.nirah_provider_version().into()),
                    notifier: (ctx.notifier.nirah_provider_identifier().into(), ctx.notifier.nirah_provider_version().into()),
                    rpc: ctx.rpc_details,
                    rpc_handler: ctx.rpc_handler_details,
                    sessions
                })
            },
            RpcRequest::GetConfig { key } => {
                if let Some(value) = ctx.config.get_config_value(&key).await? {
                    Ok(RpcResponse::Config { value })
                } else {
                    Ok(RpcResponse::InvalidConfigKey { key })
                }
            },
            RpcRequest::SetConfig { key, value } => {
                ctx.config.set_config_value(&key, Some(value)).await?;
                Ok(RpcResponse::Ok)
            },
            RpcRequest::AllAccounts => {
                Ok(RpcResponse::AllAccounts { accounts: ctx.accounts.all_accounts().await })
            },
            RpcRequest::GetAccount { id } => {
                if let Some(acc) = ctx.accounts.get_account(id).await? {
                    Ok(RpcResponse::Account { acc })
                } else {
                    Ok(RpcResponse::InvalidAccount { id })
                }
            },
            RpcRequest::CreateAccount { new } => {
                ctx.accounts.create_account(new).await?;
                Ok(RpcResponse::Ok)
            },
            RpcRequest::RemoveAccount { id } => {
                ctx.accounts.remove_account(id).await?;
                Ok(RpcResponse::Ok)
            },
            RpcRequest::EditAccount { account } => {
                ctx.accounts.edit_account(account).await?;
                Ok(RpcResponse::Ok)
            },
            RpcRequest::AllVariables => {
                Ok(RpcResponse::AllConfigVariables { vars: ctx.config.all_config_variables().await})
            },
            RpcRequest::InitializeAccount { id } => {
                if let Some(account) = ctx.accounts.get_account(id).await? {
                    match &account.ty {
                        AccountType::Sip => create_sip_session(account, &mut ctx).await
                    }
                } else {
                    Err(NirahError::InvalidAccountId(id))
                }
            },
            RpcRequest::GetContact { id } => {
                if let Some(contact) = ctx.contacts.get_contact(id).await? {
                    Ok(RpcResponse::Contact { contact })
                } else {
                    Ok(RpcResponse::InvalidContact { id })
                }
            },
            RpcRequest::CreateContact { contact } => {
                ctx.contacts.create_contact(contact).await?;
                Ok(RpcResponse::Ok)
            },
            RpcRequest::RemoveContact { id } => {
                ctx.contacts.remove_contact(id).await?;
                Ok(RpcResponse::Ok)
            },
            RpcRequest::EditContact { contact } => {
                ctx.contacts.edit_contact(contact).await?;
                Ok(RpcResponse::Ok)
            },
            RpcRequest::AllContacts => {
                Ok(RpcResponse::AllContacts { contacts: ctx.contacts.all_contacts().await })
            },
            RpcRequest::ContactTransactions { contact } => {
                if let Some(contact_data) = ctx.database.contact_transactions(contact).await {
                    Ok(RpcResponse::ContactTransactions { transactions: contact_data.clone() })
                } else {
                    Ok(RpcResponse::InvalidContact { id: contact })
                }
            },
            RpcRequest::PerformTransaction { account, contact, transaction } => {
                let contact = if let Some(contact) = ctx.contacts.get_contact(contact).await? {
                    contact
                } else {
                    return Ok(RpcResponse::InvalidContact { id: contact })
                };
                let mut sess_index = None;
                for (sess_id, sess) in ctx.sessions.iter() {
                    if sess.account_id()? == account {
                        sess_index = Some(sess_id.clone());
                    }
                }
                if let Some(sess_id) = sess_index {
                    let transaction_id = ctx.database.log(contact.id, transaction).await?;
                    let transaction = ctx.database.get_log(contact.id, transaction_id).await?.unwrap().clone();
                    let session = ctx.sessions.get_mut(&sess_id).unwrap();
                    session.handle_event(session_ctx!(ctx), SessionEvent::Transaction { transaction }).await?;
                    Ok(RpcResponse::Ok)
                } else {
                    Ok(RpcResponse::AccountSessionNotActive)
                }
            },
            RpcRequest::AcceptInvite { account, invite } => {
                if let Some(account) = ctx.accounts.get_account(account).await? {
                    for (id, session) in ctx.sessions {
                        if &account.id == id {
                            session.handle_event(session_ctx!(ctx), SessionEvent::AcceptInvite { invite }).await?;
                        }
                    }
                }

                Ok(RpcResponse::Ok)
            },
            RpcRequest::AllCurrentStreams => {
                Ok(RpcResponse::AllStreams { streams: ctx.streaming.list_streams().await? })
            },
            RpcRequest::EndCall { account, call } => {
                if let Some(account) = ctx.accounts.get_account(account).await? {
                    for (id, session) in ctx.sessions {
                        if &account.id == id {
                            let rpc_req = SessionEvent::Bye { call: (&call).into() };
                            session.handle_event(session_ctx!(ctx), rpc_req).await?;
                        }
                    }
                }
                ctx.streaming.end_stream(streaming_ctx!(ctx), call).await?;
                Ok(RpcResponse::Ok)
            }
        }
    }
}

async fn create_sip_session<'a>(account: Account, ctx: &mut ServerCtx<'a>) -> NirahResult<RpcResponse> {
    let mut sip_session = SipSessionProvider::new();
    let id = account.id;
    sip_session.connect(account, ctx).await?;
    ctx.sessions.insert(id, Box::new(sip_session));
    Ok(RpcResponse::Ok)
}
