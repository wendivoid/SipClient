use async_trait::async_trait;
use telecom::Telecom;
use telecom::Stream;

use nirah_core::prelude::*;

pub struct GSTreamerProvider {
     telecom: Telecom
}

impl GSTreamerProvider {

    pub fn new() -> NirahResult<GSTreamerProvider> {
        Ok(GSTreamerProvider {
            telecom: Telecom::new().unwrap()
        })
    }
}

impl Provider for GSTreamerProvider {
    fn nirah_provider_identifier(&self) -> &'static str {
        "GSTreamerStreamingProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn required_config_variables(&self) -> NirahResult<Vec<ConfigDefinition>> {
        Ok(vec![])
    }
}

#[async_trait]
impl StreamingProvider for GSTreamerProvider {
    async fn list_streams(&self) -> NirahResult<Vec<String>> {
        Ok(vec![])
    }

    async fn handle_streams<'a>(&mut self, _ctx: StreamingCtx<'a>, events: Vec<StreamingEvent>) -> NirahResult<()> {
        for event in events {
            debug!("Creating Stream: {:?}", event);
            match event {
                StreamingEvent::AudioSession {
                    call_id, local_port, remote_port,
                    remote_addr, codec, clock_rate,
                    identifier
                } => {
                    let stream = Stream::new(local_port as i32, remote_port as i32, remote_addr)
                                    .set_media_type("audio")
                                    .set_clock_rate(clock_rate as i32)
                                    .set_payload(identifier.0 as i32)
                                    .set_encoding_name(format!("{}",codec));
                    self.telecom.insert_stream(&call_id, stream).expect("Failed to insert telecom stream");
                }
            }
        }
        Ok(())
    }

    async fn end_stream<'a>(&mut self, _ctx: StreamingCtx<'a>, _call_id: String) -> NirahResult<()> {
        Ok(())
    }
}
