use async_trait::async_trait;

use crate::prelude::*;
use libsdp::SdpOffer;

mod null;
pub use self::null::NullStreamingProvider;

pub struct StreamingEvent {
    pub local_port: u32,
    pub call_id: String,
    pub inputs: Vec<SdpOffer>,
    pub outputs: Vec<SdpOffer>
}

#[derive(Debug, PartialEq, Clone)]
pub enum StreamingError {
    FailedOpeningSink,
    UnknownCodec(libsdp::SdpCodecIdentifier),
    InvalidMediaFormat,
    NoConnectionAddress
}

#[async_trait]
pub trait StreamingProvider: Provider {

    async fn list_streams(&self) -> NirahResult<Vec<String>>;

    async fn handle_streams<'a>(&mut self, ctx: StreamingCtx<'a>, events: StreamingEvent) -> NirahResult<()>;

    async fn end_stream<'a>(&mut self, ctx: StreamingCtx<'a>, call_id: String) -> NirahResult<()>;
}
