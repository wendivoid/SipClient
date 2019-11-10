use async_trait::async_trait;

use crate::prelude::*;

pub struct NullStreamingProvider;

impl Provider for NullStreamingProvider {

    fn nirah_provider_identifier(&self) -> &'static str {
        "NullStreamProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    fn required_config_variables(&self) -> NirahResult<Vec<(VariableKey, Option<VariableValue>)>> {
        Ok(vec![])
    }
}

#[async_trait]
impl StreamingProvider for NullStreamingProvider {
    async fn handle_session<'a>(&mut self, _ctx: StreamingCtx<'a>) -> NirahResult<()> {
        Ok(())
    }
}
