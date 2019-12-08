use async_trait::async_trait;

use crate::core::NirahResult;
use crate::core::Provider;

mod memory;
pub use self::memory::InMemoryDatabaseProvider;

mod transaction_data;
pub use self::transaction_data::TransactionEventData;
pub use self::transaction_data::NewTransactionEventData;

mod transaction_event;
pub use self::transaction_event::TransactionEvent;
pub use self::transaction_event::NewTransactionEvent;

#[async_trait]
pub trait DatabaseProvider: Provider {

    async fn contact_transactions(&mut self, _account: u32, _contact: u32) -> Option<&Vec<TransactionEvent>>;

    async fn account_transactions(&mut self, _account: u32) -> NirahResult<Vec<(u32, Option<TransactionEvent>)>>;

    async fn log(&mut self, _account: u32, _contact: u32, _log: NewTransactionEvent) -> NirahResult<u32>;

    async fn get_log(&self, _account: u32, _contact: u32, _log: u32) -> NirahResult<Option<&TransactionEvent>>;

    async fn remove_log(&mut self, _account: u32, _contact: u32, _log: u32) -> NirahResult<()>;
}
