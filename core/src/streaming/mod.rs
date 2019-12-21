use async_trait::async_trait;

use crate::prelude::*;

mod null;
pub use self::null::NullStreamingProvider;

mod event;
pub use self::event::StreamingEvent;

mod error;
pub use self::error::StreamingError;

#[async_trait]
pub trait StreamingProvider: Provider {

    async fn list_streams(&self) -> NirahResult<Vec<String>>;

    async fn handle_streams<'a>(&mut self, ctx: StreamingCtx<'a>, events: Vec<StreamingEvent>) -> NirahResult<()>;

    async fn end_stream<'a>(&mut self, ctx: StreamingCtx<'a>, call_id: String) -> NirahResult<()>;
}
