use gstreamer::State;
use gstreamer::Pipeline as GstPipeline;
use gstreamer::GstBinExtManual;
use gstreamer::ElementExtManual;

use crate::prelude::*;

use super::elements::Queue;
use super::elements::RtpJitterBuffer;
use super::elements::RtpDecoder;
use super::elements::RtpEncoder;
use super::elements::AudioSink;
use super::elements::AudioSource;
use super::elements::AudioEncoder;
use super::elements::AudioDecoder;
use super::elements::AudioResample;
use super::elements::AudioConvert;
use super::elements::UdpSink;
use super::elements::UdpSource;

pub enum Pipeline {
    VoiceInput {
        input: UdpSource,
        jitter: RtpJitterBuffer,
        rtp: RtpDecoder,
        audio: AudioDecoder,
        queue: Queue,
        converter: AudioConvert,
        resampler: AudioResample,
        output: AudioSink
    },
    VoiceOutput {
        input: AudioSource,
        audio: AudioEncoder,
        rtp: RtpEncoder,
        queue: Queue,
        output: UdpSink
    }
}

impl Pipeline {

    pub fn initialize(&self, pipeline: &GstPipeline) -> Result<(), StreamingError> {
        match self {
            Pipeline::VoiceInput { input, jitter, rtp, audio, queue, converter, resampler, output } => {
                let elements = &[&input.0, &jitter.0, &rtp.0, &audio.0, &queue.0, &converter.0, &resampler.0, &output.0];
                pipeline.add_many(elements)?;
                gstreamer::Element::link_many(elements)?;
                pipeline.set_state(State::Playing)?;
            },
            Pipeline::VoiceOutput { input, audio, rtp, queue, output } => {
                let elements = &[&input.0, &audio.0, &rtp.0, &queue.0, &output.0];
                pipeline.add_many(elements)?;
                gstreamer::Element::link_many(elements)?;
                pipeline.set_state(State::Playing)?;
            }
        }
        Ok(())
    }
}
