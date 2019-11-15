use async_trait::async_trait;

use crate::prelude::*;
use nirah_sdp::SdpOffer;

mod null;
pub use self::null::NullStreamingProvider;

#[cfg(feature = "gstreamer")]
mod gstreamer;
#[cfg(feature = "gstreamer")]
pub use self::gstreamer::GStreamerProvider;

pub struct StreamingEvent {
    pub inputs: Vec<SdpOffer>,
    pub outputs: Vec<SdpOffer>
}

#[derive(Debug, PartialEq, Clone)]
pub enum StreamingError {
    FailedOpeningSink,
    InvalidMediaFormat
}

#[async_trait]
pub trait StreamingProvider: Provider {
    async fn handle_session<'a>(&mut self, ctx: StreamingCtx<'a>, events: StreamingEvent) -> NirahResult<()>;
}
