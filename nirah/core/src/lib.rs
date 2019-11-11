#[macro_use]
extern crate log;
extern crate tokio;
extern crate serde_json;
extern crate async_trait;
extern crate nirah_sip;
extern crate nirah_uri;
extern crate nom;
extern crate futures;

#[macro_use]
pub mod config;
#[macro_use]
pub mod server;
pub mod audio;
pub mod accounts;
pub mod contacts;
pub mod core;
pub mod rpc;
pub mod session;
pub mod database;
pub mod notifier;
pub mod streaming;

pub mod prelude {
    pub use crate::core::RpcFuture;
    pub use crate::core::ConfigFuture;
    pub use crate::core::RpcHandlerFuture;
    pub use crate::core::AccountsFuture;
    pub use crate::core::ContactsFuture;
    pub use crate::core::DatabaseFuture;
    pub use crate::core::AudioFuture;
    pub use crate::core::NotifierFuture;
    pub use crate::core::SessionFuture;
    pub use crate::core::StreamingFuture;
    pub use crate::core::Provider;
    pub use crate::core::NirahResult;
    pub use crate::core::NirahError;
    pub use crate::audio::AudioProvider;
    pub use crate::audio::AudioDevice;
    pub use crate::audio::AudioDirection;
    pub use crate::accounts::Account;
    pub use crate::accounts::NewAccount;
    pub use crate::accounts::AccountType;
    pub use crate::accounts::AccountsProvider;
    pub use crate::accounts::InMemoryAccountsProvider;
    pub use crate::config::keys;
    pub use crate::config::ConfigProvider;
    pub use crate::config::VariableValue;
    pub use crate::config::VariableKey;
    pub use crate::config::InMemoryConfigProvider;
    pub use crate::contacts::ContactsProvider;
    pub use crate::contacts::Contact;
    pub use crate::contacts::NewContact;
    pub use crate::contacts::InMemoryContactsProvider;
    pub use crate::database::DatabaseProvider;
    pub use crate::database::TransactionEvent;
    pub use crate::database::TransactionEventData;
    pub use crate::database::NewTransactionEvent;
    pub use crate::database::NewTransactionEventData;
    pub use crate::database::InMemoryDatabaseProvider;
    pub use crate::notifier::NotifierProvider;
    pub use crate::notifier::NotifierParams;
    pub use crate::notifier::NullNotifierProvider;
    pub use crate::notifier::NotifierArgument;
    pub use crate::rpc::RpcProvider;
    pub use crate::rpc::RpcHandlerProvider;
    pub use crate::rpc::RpcRequest;
    pub use crate::rpc::RpcResponse;
    pub use crate::rpc::DefaultRpcHandler;
    pub use crate::server::Server;
    pub use crate::server::Builder;
    pub use crate::server::ServerCtx;
    pub use crate::server::SessionCtx;
    pub use crate::server::StreamingCtx;
    pub use crate::server::AddressManager;
    pub use crate::session::SessionProvider;
    pub use crate::session::SipSessionProvider;
    pub use crate::session::SessionEvent;
    pub use crate::streaming::StreamingProvider;
    pub use crate::streaming::NullStreamingProvider;
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
