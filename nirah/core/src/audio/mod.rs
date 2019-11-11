use async_trait::async_trait;
use serde::{ Serialize, Deserialize };

use crate::prelude::*;

use std::fmt;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum AudioDirection {
    Playback,
    Capture,
    Both
}

impl fmt::Display for AudioDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AudioDirection::Playback => write!(f, "Playback"),
            AudioDirection::Capture => write!(f, "Capture"),
            AudioDirection::Both => write!(f, "Both")
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub direction: AudioDirection,
    pub name: Option<String>,
    pub description: Option<String>
}

#[async_trait]
pub trait AudioProvider: Provider {

    async fn list_audio_devices(&mut self) -> NirahResult<Vec<AudioDevice>>;

    async fn start_ringing<'a>(&mut self, cfg: &mut ConfigFuture) -> NirahResult<()>;

    async fn stop_ringing<'a>(&mut self, cfg: &mut ConfigFuture) -> NirahResult<()>;
}
