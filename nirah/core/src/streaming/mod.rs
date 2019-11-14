use async_trait::async_trait;

use crate::prelude::*;
use nirah_sdp::SdpOffer;

mod null;
pub use self::null::NullStreamingProvider;

pub struct StreamingEvent {
    pub inputs: Vec<SdpOffer>,
    pub outputs: Vec<SdpOffer>
}

#[async_trait]
pub trait StreamingProvider: Provider {
    async fn handle_session<'a>(&mut self, ctx: StreamingCtx<'a>, events: StreamingEvent) -> NirahResult<()>;
}
