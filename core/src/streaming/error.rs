#[derive(Debug, PartialEq, Clone)]
pub enum StreamingError {
    FailedOpeningSink,
    UnknownCodec(libsdp::SdpCodecIdentifier),
    InvalidMediaFormat,
    NoConnectionAddress
}
