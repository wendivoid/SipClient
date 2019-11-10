use async_trait::async_trait;

use nirah_core::prelude::*;

pub struct GStreamerProvider {

}

impl GStreamerProvider {

    pub fn new() -> GStreamerProvider {
        GStreamerProvider {

        }
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
        Ok(vec![])
    }
}

#[async_trait]
impl StreamingProvider for GStreamerProvider {
    async fn handle_session<'a>(&mut self, _ctx: StreamingCtx<'a>) -> NirahResult<()> {
        Ok(())
    }
}
