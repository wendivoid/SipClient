use futures::future;
use futures::future::Either;
use futures::FutureExt;
use tokio::time::delay_until;

use crate::prelude::*;

use super::AddressManager;
use super::ServerCtx;

use std::collections::HashMap;

pub enum ServerEvent<T> {
    None,
    Timeout(u32),
    RpcRequest(RpcRequest, T),
    RecievedData(u32, Vec<u8>)
}

pub struct Server<T> {
    pub accounts: AccountsFuture,
    pub config: ConfigFuture,
    pub contacts: ContactsFuture,
    pub database: DatabaseFuture,
    pub notifier: NotifierFuture,
    pub rpc: RpcFuture<T>,
    pub rpc_handler: RpcHandlerFuture,
    pub streaming: StreamingFuture,
    pub address_manager: AddressManager,
    pub sessions: HashMap<u32, SessionFuture>
}

impl <T>Server<T> {

    async fn initialize_config(&mut self) -> NirahResult<()> {
        self.config.register_config_settings(&self.config.required_config_variables()?).await?;
        self.config.register_config_settings(&self.accounts.required_config_variables()?).await?;
        self.config.register_config_settings(&self.rpc.required_config_variables()?).await?;
        self.config.register_config_settings(&self.rpc_handler.required_config_variables()?).await?;
        self.config.register_config_settings(&self.notifier.required_config_variables()?).await?;
        self.config.register_config_settings(&self.streaming.required_config_variables()?).await?;
        self.config.register_config_settings(&SipSessionProvider::new().required_config_variables()?).await?;
        Ok(())
    }

    pub async fn mainloop(mut self) -> NirahResult<()> {
        self.initialize_config().await?;
        self.rpc.connect(&mut self.config).await?;
        loop {
            match self.run_once().await {
                Ok(ServerEvent::RpcRequest(msg, peer)) => {
                   debug!("Rpc Message: {:?}", msg);
                   let res = self.rpc_handler.handle(msg, ctx!(self)).await?;
                   self.rpc.send(res, peer).await?;
                },
                Ok(ServerEvent::Timeout(acc)) => {
                    debug!("Account({}) timed out", acc);
                    if let Some(session) = self.sessions.get_mut(&acc) {
                        session.handle_event(session_ctx_from_server!(self), SessionEvent::Timeout).await?;
                    } else {
                        warn!("Received a timeout event for an unknown session.")
                    }
                },
                Ok(ServerEvent::RecievedData(id, data)) => {
                    if !self.sessions.contains_key(&id) {
                        warn!("Received data for a session that didn't exist");
                        continue;
                    }
                    let mut session = self.sessions.remove(&id).unwrap();
                    session.handle_event(session_ctx_from_server!(self), SessionEvent::Data { data }).await?;
                    self.sessions.insert(id, session);
                },
                Ok(ServerEvent::None) => {},
                Err(err) => error!("{:?}", err)
            }
        }
    }

    pub async fn run_once(&mut self) -> NirahResult<ServerEvent<T>> {
        // let (msg, peer) = self.rpc.get().await?;
        // let res = self.rpc_handler.handle(msg, ctx!(self)).await?;
        // self.rpc.send(res, peer).await?;
         if self.sessions.len() == 0 {
             return self.run_rpc_only().await;
         }
         let mut timeouts = vec![];
         let mut messages = vec![];
         for (id, sess) in &mut self.sessions {
             let id = id.clone();
             let id2 = id.clone();
             if let Some(timeout_length) = sess.timeout_time().await? {
                 let timeout_future = delay_until(timeout_length).map(move |_| id);
                 timeouts.push(timeout_future);
             }
             let read_future = sess.read_future().map(move |data| (id2, data));
             messages.push(read_future);
         }
         if timeouts.len() == 0 {
             let sess_future = future::select_all(messages);
             match future::select(self.rpc.get(), sess_future).await {
                 Either::Left((Ok((msg, peer)), _)) => {
                     Ok(ServerEvent::RpcRequest(msg, peer))
                 },
                 _ => Ok(ServerEvent::None)
             }
         } else {
             let sess_future = future::select(
                 future::select_all(timeouts),
                 future::select_all(messages)
             );
             match future::select(self.rpc.get(), sess_future).await {
                 Either::Left((Ok((msg, peer)), _)) => {
                     Ok(ServerEvent::RpcRequest(msg, peer))
                 },
                 Either::Right((Either::Left(((acc, _, _), _)), _)) => Ok(ServerEvent::Timeout(acc.clone())),
                 Either::Right((Either::Right((((acc, Ok(data)), _, _), _)), _)) => Ok(ServerEvent::RecievedData(acc.clone(), data)),
                 _ => Ok(ServerEvent::None)
             }
         }
    }

    pub async fn run_rpc_only(&mut self) -> NirahResult<ServerEvent<T>> {
        let (msg, peer) = self.rpc.get().await?;
        Ok(ServerEvent::RpcRequest(msg, peer))
    }
}
