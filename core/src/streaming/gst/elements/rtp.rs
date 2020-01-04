use gstreamer::Element;
use gstreamer::ElementFactory;

use crate::prelude::*;
use libsdp::SdpEncoding;

pub struct RtpDecoder(pub Element);

impl RtpDecoder {

    pub fn new(encoding: &SdpEncoding) -> Result<RtpDecoder, StreamingError> {
        let elem = match encoding {
            SdpEncoding::Pcmu => ElementFactory::make("rtppcmudepay", None)?,
            SdpEncoding::Pcma => ElementFactory::make("rtppcmadepay", None)?,
            SdpEncoding::Unknown(other) => panic!("Unknown encoding: {}", other)
        };
        Ok(RtpDecoder(elem))
    }
}

pub struct RtpEncoder(pub Element);

impl RtpEncoder {

    pub fn new(encoding: &SdpEncoding) -> Result<RtpEncoder, StreamingError> {
        let elem = match encoding {
            SdpEncoding::Pcmu => ElementFactory::make("rtppcmupay", None)?,
            SdpEncoding::Pcma => ElementFactory::make("rtppcmapay", None)?,
            SdpEncoding::Unknown(other) => panic!("Unknown encoding: {}", other)
        };
        Ok(RtpEncoder(elem))
    }
}
