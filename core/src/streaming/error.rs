#[cfg(feature = "gst")]
use glib::error::BoolError;
#[cfg(feature = "gst")]
use gstreamer::StateChangeError;

#[derive(Debug, Clone)]
pub enum StreamingError {
    FailedOpeningSink,
    UnknownCodec(libsdp::SdpCodecIdentifier),
    InvalidMediaFormat,
    NoConnectionAddress,
    #[cfg(feature = "gst")]
    GBool(BoolError),
    #[cfg(feature = "gst")]
    StateChange(StateChangeError)
}

#[cfg(feature = "gst")]
impl From<BoolError> for StreamingError {
    fn from(err: BoolError) -> StreamingError {
        StreamingError::GBool(err)
    }
}

#[cfg(feature = "gst")]
impl From<StateChangeError> for StreamingError {
    fn from(err: StateChangeError) -> StreamingError {
        StreamingError::StateChange(err)
    }
}
