use libsdp::SdpOffer;

#[derive(Debug, PartialEq, Clone)]
pub struct StreamingEvent {
    pub local_port: u32,
    pub call_id: String,
    pub inputs: Vec<SdpOffer>,
    pub outputs: Vec<SdpOffer>
}
