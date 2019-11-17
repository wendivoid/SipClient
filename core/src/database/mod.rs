use async_trait::async_trait;

use crate::core::NirahResult;
use crate::core::Provider;

mod memory;
pub use self::memory::InMemoryDatabaseProvider;

mod models;
pub use self::models::TransactionEvent;
pub use self::models::TransactionEventData;
pub use self::models::NewTransactionEvent;
pub use self::models::NewTransactionEventData;

#[async_trait]
pub trait DatabaseProvider: Provider {

    async fn contact_transactions(&mut self, _contact: u32) -> Option<&Vec<TransactionEvent>>;

    async fn log(&mut self, _contact: u32, _log: NewTransactionEvent) -> NirahResult<u32>;

    async fn get_log(&self, _contact: u32, _log: u32) -> NirahResult<Option<&TransactionEvent>>;

    async fn remove_log(&mut self, _contact: u32, _log: u32) -> NirahResult<()>;
}
