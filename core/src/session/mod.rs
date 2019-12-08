use async_trait::async_trait;

use crate::prelude::*;

use tokio::time::Instant;

mod sip;
pub use self::sip::SipSessionProvider;

mod event;
pub use self::event::SessionEvent;

#[async_trait]
pub trait SessionProvider: Provider {

    fn account_id(&self) -> NirahResult<u32>;

    async fn read_future<'a>(&mut self) -> NirahResult<Vec<u8>>;

    async fn timeout_time(&self) -> NirahResult<Option<Instant>>;

    async fn handle_event<'a>(&mut self, ctx: SessionCtx<'a>, event: SessionEvent) -> NirahResult<()>;
}
