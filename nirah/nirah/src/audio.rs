use async_trait::async_trait;

use nirah_core::prelude::*;
use alsa::Direction as AlsaDirection;
use alsa::device_name::HintIter;

use std::ffi::CString;

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

    async fn list_audio_devices(&mut self) -> NirahResult<Vec<AudioDevice>> {
        let mut output = vec![];
        for t in &["pcm"] {
            let i = HintIter::new(None, &*CString::new(*t).unwrap()).unwrap();
            for a in i {
                let direction = match a.direction {
                    None => AudioDirection::Both,
                    Some(AlsaDirection::Playback) => AudioDirection::Playback,
                    Some(AlsaDirection::Capture) => AudioDirection::Capture
                };
                output.push(AudioDevice { direction, name: a.name, description: a.desc });
            }
        }
        Ok(output)
    }

    async fn start_ringing<'a>(&mut self, _cfg: &mut ConfigFuture) -> NirahResult<()> {
        Ok(())
    }

    async fn stop_ringing<'a>(&mut self, _cfg: &mut ConfigFuture) -> NirahResult<()> {
        Ok(())
    }
}
