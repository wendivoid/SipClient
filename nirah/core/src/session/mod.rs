use async_trait::async_trait;

use crate::prelude::*;

use std::time::Instant;

mod sip;
pub use self::sip::SipSessionProvider;

pub enum SessionEvent {
    Timeout,
    AcceptInvite { invite: usize },
    Transaction { transaction: TransactionEvent },
    Data { data: Vec<u8> }
}

#[async_trait]
pub trait SessionProvider: Provider {

    fn account_id(&self) -> NirahResult<u32>;

    async fn read_future<'a>(&mut self) -> NirahResult<Vec<u8>>;

    async fn timeout_time(&self) -> NirahResult<Option<Instant>>;

    async fn handle_event<'a>(&mut self, ctx: SessionCtx<'a>, event: SessionEvent) -> NirahResult<()>;
}
