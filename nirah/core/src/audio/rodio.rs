use async_trait::async_trait;

use crate::prelude::*;

pub struct RodioAudioProvider {

}

impl RodioAudioProvider {

    pub fn new() -> RodioAudioProvider {
        RodioAudioProvider {

        }
    }
}

impl Provider for RodioAudioProvider {

    fn nirah_provider_identifier(&self) -> &'static str {
        "RodioAudioProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn required_config_variables(&self) -> NirahResult<Vec<(VariableKey, Option<VariableValue>)>> {
        Ok(vec![])
    }
}


#[async_trait]
impl AudioProvider for RodioAudioProvider {

    async fn start_ringing<'a>(&mut self, _cfg: &mut ConfigFuture) -> NirahResult<()> {
        Ok(())
    }

    async fn stop_ringing<'a>(&mut self, _cfg: &mut ConfigFuture) -> NirahResult<()> {
        Ok(())
    }
}
