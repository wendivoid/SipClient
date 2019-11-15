use async_trait::async_trait;

use crate::prelude::*;


pub struct GStreamerProvider {

}

impl GStreamerProvider {

    pub fn new() -> NirahResult<GStreamerProvider> {
        gstreamer::init()?;
        Ok(GStreamerProvider {

        })
    }
}

impl Provider for GStreamerProvider {

    fn nirah_provider_identifier(&self) -> &'static str {
        "GStreamerProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[async_trait]
impl StreamingProvider for GStreamerProvider {
    async fn handle_session<'a>(&mut self, _ctx: StreamingCtx<'a>, _event: StreamingEvent) -> NirahResult<()> {
        Ok(())
    }
}
