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
    fn required_config_variables(&self) -> NirahResult<Vec<(VariableKey, Option<VariableValue>)>> {
        Ok(vec![
            (VariableKey::new("audio_output_device"), Some(VariableValue::String("default".into())))
        ])
    }
}

#[async_trait]
impl StreamingProvider for GStreamerProvider {
    async fn handle_session<'a>(&mut self, mut _ctx: StreamingCtx<'a>, _event: StreamingEvent) -> NirahResult<()> {
        Ok(())
    }
}
