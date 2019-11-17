mod provider;
pub use self::provider::Provider;

mod error;
pub use self::error::NirahError;

pub type NirahResult<T> = Result<T, NirahError>;

pub type ConfigFuture = Box<dyn crate::config::ConfigProvider>;
pub type ContactsFuture = Box<dyn crate::contacts::ContactsProvider>;
pub type AccountsFuture = Box<dyn crate::accounts::AccountsProvider>;
pub type DatabaseFuture = Box<dyn crate::database::DatabaseProvider>;
pub type NotifierFuture = Box<dyn crate::notifier::NotifierProvider>;
pub type RpcFuture<T> = Box<dyn crate::rpc::RpcProvider<T>>;
pub type RpcHandlerFuture = Box<dyn crate::rpc::RpcHandlerProvider>;
pub type SessionFuture = Box<dyn crate::session::SessionProvider>;
