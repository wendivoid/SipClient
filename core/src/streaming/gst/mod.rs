use async_trait::async_trait;
use gstreamer::Caps;
use gstreamer::State;
use gstreamer::ElementExtManual;
use gstreamer::GstBinExtManual;
use libsdp::SdpEncoding;
use libsdp::SdpCodecIdentifier;

use crate::prelude::*;

mod elements;
mod pipeline;
pub use self::pipeline::Pipeline;
mod session;
pub use self::session::StreamingSession;

pub struct GStreamerProvider {
    sessions: Vec<StreamingSession>
}

impl GStreamerProvider {

    pub fn new() -> NirahResult<GStreamerProvider> {
        gstreamer::init().expect("Failed to initialize gstreamer");
        Ok(GStreamerProvider {
           sessions: vec![]
        })
    }
}

impl Provider for GStreamerProvider {
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
impl StreamingProvider for GStreamerProvider {
    async fn list_streams(&self) -> NirahResult<Vec<String>> {
        Ok(vec![])
    }

    async fn handle_streams<'a>(&mut self, _ctx: StreamingCtx<'a>, events: Vec<StreamingEvent>) -> NirahResult<()> {
        for event in events {
            match event {
                StreamingEvent::AudioSession {
                    call_id, local_port, remote_addr, remote_port,
                    codec, identifier, clock_rate
                } => {
                    let pipeline = gstreamer::Pipeline::new(None);
                    let input_pipeline = create_input_pipeline(&pipeline, local_port as i32, &codec, &identifier, clock_rate as i32)?;
                    let output_pipeline = create_output_pipeline(&pipeline, &remote_addr, remote_port as i32, &codec, &identifier, clock_rate as i32)?;
                    if let Err(err) = pipeline.set_state(State::Playing) {
                        return Err(NirahError::Streaming(StreamingError::StateChange(err)));
                    }
                    pipeline.debug_to_dot_file(gstreamer::DebugGraphDetails::all(), "PLAYING");
                    let session = StreamingSession {
                        session_id: call_id,
                        pipelines: vec![input_pipeline, output_pipeline],
                        pipeline
                    };
                    self.sessions.push(session);
                }
            }
        }
        Ok(())
    }

    async fn end_stream<'a>(&mut self, _ctx: StreamingCtx<'a>, _call_id: String) -> NirahResult<()> {
        Ok(())
    }
}

fn create_input_pipeline(pipeline: &gstreamer::Pipeline, local_port: i32, codec: &SdpEncoding, identifier: &SdpCodecIdentifier, clock_rate: i32) -> Result<Pipeline, StreamingError> {
    let caps = Caps::new_simple("application/x-rtp", &[
        ("media", &"audio"),
        ("payload", &identifier.0),
        ("clock-rate", &clock_rate)
    ]);
    let udp_source = elements::UdpSource::new(local_port, caps)?;
    let jitter = elements::RtpJitterBuffer::new()?;
    let rtp_decoder = elements::RtpDecoder::new(&codec)?;
    let audio_decoder = elements::AudioDecoder::new(&codec)?;
    let queue = elements::Queue::new()?;
    let converter = elements::AudioConvert::new()?;
    let resampler = elements::AudioResample::new()?;
    let audio_output = elements::AudioSink::new()?;
    let _pipeline = Pipeline::VoiceInput {
        input: udp_source,
        jitter: jitter,
        rtp: rtp_decoder,
        audio: audio_decoder,
        queue: queue,
        output: audio_output,
        converter,
        resampler
    };
    _pipeline.initialize(&pipeline)?;
    Ok(_pipeline)
}

fn create_output_pipeline(pipeline: &gstreamer::Pipeline, remote_addr: &str, remote_port: i32, codec: &SdpEncoding, identifier: &SdpCodecIdentifier, clock_rate: i32) -> Result<Pipeline, StreamingError> {
    let audio_input = elements::AudioSource::new()?;
    let audio_encoder = elements::AudioEncoder::new(&codec)?;
    let rtp_encoder = elements::RtpEncoder::new(&codec)?;
    let queue = elements::Queue::new()?;
    let udp_sink = elements::UdpSink::new(remote_addr, remote_port)?;
    let _pipeline = Pipeline::VoiceOutput {
        input: audio_input,
        audio: audio_encoder,
        rtp: rtp_encoder,
        queue: queue,
        output: udp_sink
    };
    _pipeline.initialize(&pipeline)?;
    Ok(_pipeline)
}
