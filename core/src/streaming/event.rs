use libsdp::SdpEncoding;
use libsdp::SdpCodecIdentifier;
// #[derive(Debug, PartialEq, Clone)]
// pub struct StreamingEvent {
//     pub local_port: u32,
//     pub call_id: String,
//     pub inputs: Vec<SdpOffer>,
//     pub outputs: Vec<SdpOffer>
// }

#[derive(Debug, PartialEq, Clone)]
pub enum StreamingEvent {
    AudioSession {
        call_id: String,
        local_port: u32,
        remote_addr: String,
        remote_port: u32,
        codec: SdpEncoding,
        identifier: SdpCodecIdentifier,
        clock_rate: u64
    }
}
