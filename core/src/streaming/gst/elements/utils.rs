use gstreamer::Element;
use gstreamer::ElementFactory;

use crate::prelude::*;

pub struct RtpJitterBuffer(pub Element);

impl RtpJitterBuffer {

    pub fn new() -> Result<RtpJitterBuffer, StreamingError> {
        Ok(RtpJitterBuffer(ElementFactory::make("rtpjitterbuffer", None)?))
    }
}

pub struct Queue(pub Element);

impl Queue {

    pub fn new() -> Result<Queue, StreamingError> {
        Ok(Queue(ElementFactory::make("queue", None)?))
    }
}


pub struct AudioConvert(pub Element);

impl AudioConvert {

    pub fn new() -> Result<AudioConvert, StreamingError> {
        Ok(AudioConvert(ElementFactory::make("audioconvert", None)?))
    }
}

pub struct AudioResample(pub Element);

impl AudioResample {

    pub fn new() -> Result<AudioResample, StreamingError> {
        Ok(AudioResample(ElementFactory::make("audioconvert", None)?))
    }
}
