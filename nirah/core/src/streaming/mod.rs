use async_trait::async_trait;

use crate::prelude::*;

mod null;
pub use self::null::NullStreamingProvider;

#[async_trait]
pub trait StreamingProvider: Provider {
    async fn handle_session<'a>(&mut self, ctx: StreamingCtx<'a>) -> NirahResult<()>;
}
