use glib::ObjectExt;
use glib::ToValue;
use gstreamer::Element;
use gstreamer::ElementFactory;

use libsdp::SdpEncoding;

use crate::streaming::StreamingError;

pub struct AudioSource(pub Element);

impl AudioSource {

    pub fn new() -> Result<AudioSource, StreamingError> {
        let elem = ElementFactory::make("pulsesrc", None)?;
        elem.set_property("device", &"alsa_input.pci-0000_00_1f.3.analog-stereo".to_value())?;
        Ok(AudioSource(elem))
    }
}

pub struct AudioSink(pub Element);

impl AudioSink {

    pub fn new() -> Result<AudioSink, StreamingError> {
        let elem = ElementFactory::make("pulsesink", None)?;
        elem.set_property("device", &"alsa_output.pci-0000_00_1f.3.analog-stereo".to_value())?;
        elem.set_property("async", &false.to_value())?;
        elem.set_property("sync", &false.to_value())?;
        Ok(AudioSink(elem))
    }
}

pub struct AudioDecoder(pub Element);

impl AudioDecoder {

    pub fn new(encoding: &SdpEncoding) -> Result<AudioDecoder, StreamingError> {
        let elem = match encoding {
            SdpEncoding::Pcmu => ElementFactory::make("mulawdec", None)?,
            SdpEncoding::Pcma => ElementFactory::make("alawdec", None)?,
            SdpEncoding::Unknown(other) => panic!("Unknown encoding: {}", other)
        };
        Ok(AudioDecoder(elem))
    }
}

pub struct AudioEncoder(pub Element);

impl AudioEncoder {

    pub fn new(encoding: &SdpEncoding) -> Result<AudioEncoder, StreamingError> {
        let elem = match encoding {
            SdpEncoding::Pcmu => ElementFactory::make("mulawenc", None)?,
            SdpEncoding::Pcma => ElementFactory::make("alawenc", None)?,
            SdpEncoding::Unknown(other) => panic!("Unknown encoding: {}", other)
        };
        Ok(AudioEncoder(elem))
    }
}
