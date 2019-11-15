use serde::{ Serialize, Deserialize };

use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
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
