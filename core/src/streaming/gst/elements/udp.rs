use glib::ToValue;
use glib::ObjectExt;
use gstreamer::Caps;
use gstreamer::Element;
use gstreamer::ElementFactory;

use crate::prelude::*;

pub struct UdpSink(pub Element);

impl UdpSink {

    pub fn new<S: AsRef<str>>(host: S, port: i32) -> Result<UdpSink, StreamingError> {
        let elem = ElementFactory::make("udpsink", None)?;
        elem.set_property("host", &host.as_ref().to_value())?;
        elem.set_property("port", &port.to_value())?;
        elem.set_property("sync", &false.to_value())?;
        elem.set_property("async", &false.to_value())?;
        Ok(UdpSink(elem))
    }
}

pub struct UdpSource(pub Element);

impl UdpSource {

    pub fn new(port: i32, caps: Caps) -> Result<UdpSource, StreamingError> {
        let elem = ElementFactory::make("udpsrc", None)?;
        elem.set_property("port", &port.to_value())?;
        elem.set_property("caps", &caps.to_value())?;
        Ok(UdpSource(elem))
    }
}
