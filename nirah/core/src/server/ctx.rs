use crate::prelude::*;

use std::collections::HashMap;

pub struct SessionCtx<'a> {
    pub audio: &'a mut AudioFuture,
    pub notifier: &'a mut NotifierFuture,
    pub accounts: &'a mut AccountsFuture,
    pub config: &'a mut ConfigFuture,
    pub contacts: &'a mut ContactsFuture,
    pub database: &'a mut DatabaseFuture,
    pub streaming: &'a mut StreamingFuture,
}

pub struct StreamingCtx<'a> {
    pub audio: &'a mut AudioFuture,
    pub config: &'a mut ConfigFuture,
    pub notifier: &'a mut NotifierFuture,
    pub accounts: &'a mut AccountsFuture,
    pub contacts: &'a mut ContactsFuture,
    pub database: &'a mut DatabaseFuture
}

pub struct ServerCtx<'a> {
    pub audio: &'a mut AudioFuture,
    pub notifier: &'a mut NotifierFuture,
    pub accounts: &'a mut AccountsFuture,
    pub config: &'a mut ConfigFuture,
    pub contacts: &'a mut ContactsFuture,
    pub database: &'a mut DatabaseFuture,
    pub streaming: &'a mut StreamingFuture,
    pub address_manager: &'a mut AddressManager,
    pub sessions: &'a mut HashMap<u32, SessionFuture>,
    pub rpc_details: (String, String),
    pub rpc_handler_details: (String, String)
}

#[macro_export]
macro_rules! ctx {
    ($server:ident) => {
        ServerCtx {
            audio: &mut $server.audio,
            notifier: &mut $server.notifier,
            accounts: &mut $server.accounts,
            config: &mut $server.config,
            contacts: &mut $server.contacts,
            database: &mut $server.database,
            streaming: &mut $server.streaming,
            address_manager: &mut $server.address_manager,
            sessions: &mut $server.sessions,
            rpc_details: (
                $server.rpc.nirah_provider_identifier().into(),
                $server.rpc.nirah_provider_version().into()
            ),
            rpc_handler_details: (
                $server.rpc_handler.nirah_provider_identifier().into(),
                $server.rpc_handler.nirah_provider_version().into()
            )
        }
    }
}

#[macro_export]
macro_rules! session_ctx {
    ($server:ident) => {
        SessionCtx {
            audio: $server.audio,
            notifier: $server.notifier,
            accounts: $server.accounts,
            config: $server.config,
            contacts: $server.contacts,
            database: $server.database,
            streaming: $server.streaming,
        }
    }
}

#[macro_export]
macro_rules! streaming_ctx {
    ($server:ident) => {
        StreamingCtx {
            audio: $server.audio,
            notifier: $server.notifier,
            accounts: $server.accounts,
            config: $server.config,
            contacts: $server.contacts,
            database: $server.database
        }
    }
}

#[macro_export]
macro_rules! session_ctx_from_server {
    ($server:ident) => {
        SessionCtx {
            audio: &mut $server.audio,
            notifier: &mut $server.notifier,
            accounts: &mut $server.accounts,
            config: &mut $server.config,
            contacts: &mut $server.contacts,
            database: &mut $server.database,
            streaming: &mut $server.streaming,
        }
    }
}
