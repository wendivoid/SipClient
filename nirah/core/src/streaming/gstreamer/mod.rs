use async_trait::async_trait;
use gstreamer::ParseFlags;
use gstreamer::parse_launch_full;
use gstreamer::ElementExt;
use gstreamer::ElementExtManual;
use gstreamer::MessageView;
use glib::object::ObjectExt;

use crate::prelude::*;
use nirah_sdp::Codec;
use nirah_sdp::SdpMediaType;

mod run_signal;
pub use self::run_signal::RunSignal;

pub struct GStreamerProvider {
    active_streams: Vec<(String, RunSignal)>
}

impl GStreamerProvider {

    pub fn new() -> NirahResult<GStreamerProvider> {
        gstreamer::init()?;
        Ok(GStreamerProvider {
            active_streams: vec![]
        })
    }

    pub async fn get_audio_sink_element(&self, cfg: &mut ConfigFuture) -> NirahResult<String> {
        let audio_output_key = VariableKey::new("audio_output_device");
        let cfg_value = __config_get_string!(cfg, audio_output_key)?;
        Ok(format!("alsasink device={}", cfg_value))
    }

    pub async fn get_audio_source_element(&self, cfg: &mut ConfigFuture) -> NirahResult<String> {
        let audio_input_key = VariableKey::new("audio_input_device");
        let cfg_value = __config_get_string!(cfg, audio_input_key)?;
        Ok(format!("pulsesrc device=\"{}\"", cfg_value))
    }

    pub fn get_udp_source_element(&self, port: u32, codec: &Codec) -> NirahResult<String> {
        let codec_name = match codec {
            Codec::Pcmu => "PCMU",
            Codec::Pcma => "PCMA",
            _ => unreachable!()
        };
        Ok(format!("udpsrc port={} caps=\"application/x-rtp,media=(string)audio,clock-rate=(int)8000,encoding-name=(string){}\" ! queue", port, codec_name))
    }

    pub fn get_udp_sink_element(&self, address: &str, port: u32, codec: &Codec) -> NirahResult<String> {
        let codec_name = match codec {
            Codec::Pcmu => "PCMU",
            Codec::Pcma => "PCMA",
            _ => unreachable!()
        };
        Ok(
          format!(
            "udpsink host=\"{}\" port={}",
            address,
            port
        ))
    }

    pub async fn handle_input_audio_session<'a>(&mut self, ctx: &mut StreamingCtx<'a>, codec: Codec, port: u32, call_id: String) -> NirahResult<String> {
       let audio_sink = self.get_audio_sink_element(&mut ctx.config).await?;
       let udp_source = self.get_udp_source_element(port, &codec)?;
       let (encode, decode) = match codec {
         Codec::Pcmu => ("rtppcmudepay", "mulawdec"),
         Codec::Pcma => ("rtppcmadepay", "alawdec"),
         codec => {
             return Err(NirahError::Streaming(StreamingError::UnknownCodec(codec)));
         }
       };
       Ok(format!("{} ! {} ! {} ! {}", udp_source, encode, decode, audio_sink))
   }

   pub async fn handle_output_audio_session<'a>(&mut self, ctx: &mut StreamingCtx<'a>, codec: Codec, address: &str, port: u32, call_id: String) -> NirahResult<String> {
       let audio_src = self.get_audio_source_element(&mut ctx.config).await?;
       let udp_sink = self.get_udp_sink_element(address, port, &codec)?;
       let (encode, decode) = match codec {
           Codec::Pcmu => ("mulawenc", "rtppcmupay"),
           Codec::Pcma => ("alawenc", "rtppcmapay"),
           codec => {
               return Err(NirahError::Streaming(StreamingError::UnknownCodec(codec)))
           }
       };
       Ok(format!("{} ! {} ! {} ! {}", audio_src, encode, decode, udp_sink))
   }

   async fn create_session(&mut self, call_id: &str, pipestring: &str) -> NirahResult<()> {
       trace!("GSTreamer Pipline: {}", &pipestring);
       let mut parser_context = gstreamer::ParseContext::new();
       let pipeline = parse_launch_full(&pipestring, Some(&mut parser_context), ParseFlags::NONE).unwrap();
       let bus = pipeline.get_bus().unwrap();

       pipeline
           .set_state(gstreamer::State::Playing)
           .expect("Unable to set the pipeline to the `Playing` state");

       let run_signal = RunSignal::new();
       let mut signal = run_signal.clone();
       std::thread::spawn(move || {
           if signal.is_running() {
               if let Some(msg) = bus.iter_timed(gstreamer::CLOCK_TIME_NONE).next() {
                   match msg.view() {
                       MessageView::Eos(..) => {
                           warn!("EOS");
                           signal.stop()
                       },
                       MessageView::Error(err) => {
                           warn!("{:?}", err);
                           signal.stop();
                       }
                       _ => (),
                   }
               }
           } else {
               pipeline
                 .set_state(gstreamer::State::Null)
                 .expect("Unable to set the pipeline to the `Null` state");
           }

       });
       self.active_streams.push((call_id.to_string(), run_signal));
       Ok(())
    }
}

impl Provider for GStreamerProvider {

    fn nirah_provider_identifier(&self) -> &'static str {
        "GStreamerProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn required_config_variables(&self) -> NirahResult<Vec<(VariableKey, Option<VariableValue>)>> {
        Ok(vec![
            (VariableKey::new("audio_output_device"), Some(VariableValue::String("default".into()))),
            (VariableKey::new("audio_input_device"), Some(VariableValue::String("default".into())))
        ])
    }
}

#[async_trait]
impl StreamingProvider for GStreamerProvider {
    async fn handle_streams<'a>(&mut self, mut ctx: StreamingCtx<'a>, event: StreamingEvent) -> NirahResult<()> {
        //let mut pipestring = String::new();
        //'inputs: for sdp_offer in &event.inputs {
        //    for media in &sdp_offer.media {
        //        if media.media == SdpMediaType::Audio {
        //            for format in &media.formats {
        //                if format.codec == Codec::Pcmu || format.codec == Codec::Pcma {
        //                    let s = self.handle_input_audio_session(&mut ctx, format.codec.clone(), event.local_port, event.call_id.clone()).await?;
        //                    pipestring += &format!(" {}", s);
        //                    break 'inputs;
        //                }
        //            }
        //        }
        //    }
        //}
        //self.create_session(&event.call_id, &pipestring).await?;
        let mut pipestring = String::new();
        'outputs: for sdp_offer in &event.outputs {
            for media in &sdp_offer.media {
                if media.media == SdpMediaType::Audio {
                    for format in &media.formats {
                        if format.codec == Codec::Pcmu || format.codec == Codec::Pcma {
                            let address = if let Some(conn) = &format.connection {
                                conn.address.clone()
                            } else {
                                if let Some(conn) = sdp_offer.get_connection() {
                                    conn.address
                                } else {
                                    return Err(NirahError::Streaming(StreamingError::NoConnectionAddress));
                                }
                            };
                            let s = self.handle_output_audio_session(&mut ctx, format.codec.clone(), &address, media.port, event.call_id.clone()).await?;
                            pipestring += &format!(" {}", s);
                            break 'outputs;
                        } else {
                            warn!("No valid Audio codecs")
                        }
                    }
                }
            }
        }
        if pipestring.len() > 0 {
            self.create_session(&event.call_id, &pipestring).await?;
        }
        Ok(())
    }

    async fn end_stream<'a>(&mut self, _ctx: StreamingCtx<'a>, call_id: String) -> NirahResult<()> {
        let mut dexs = vec![];
        for (index, (id, stream)) in self.active_streams.iter_mut().enumerate() {
            if id == &call_id {
                stream.stop();
                dexs.push(index);
            }
        }
        for d in dexs {
            self.active_streams.remove(d);
        }
        Ok(())
    }

    async fn list_streams(&self) -> NirahResult<Vec<String>> {
        Ok(self.active_streams.iter().map(|item| item.0.clone()).collect())
    }
}
