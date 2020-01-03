use async_trait::async_trait;

use crate::prelude::*;

pub struct GStreamerProvider {

}

impl GStreamerProvider {

    pub fn new() -> NirahResult<GStreamerProvider> {
        gstreamer::init().expect("Failed to initialize gstreamer");
        Ok(GStreamerProvider {

        })
    }
}

impl Provider for GStreamerProvider {
    fn nirah_provider_identifier(&self) -> &'static str {
        "GSTreamerStreamingProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn required_config_variables(&self) -> NirahResult<Vec<ConfigDefinition>> {
        Ok(vec![])
    }
}

#[async_trait]
impl StreamingProvider for GStreamerProvider {
    async fn list_streams(&self) -> NirahResult<Vec<String>> {
        Ok(vec![])
    }

    async fn handle_streams<'a>(&mut self, _ctx: StreamingCtx<'a>, _events: Vec<StreamingEvent>) -> NirahResult<()> {
        Ok(())
    }

    async fn end_stream<'a>(&mut self, _ctx: StreamingCtx<'a>, _call_id: String) -> NirahResult<()> {
        Ok(())
    }
}
