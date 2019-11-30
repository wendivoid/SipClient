use async_trait::async_trait;
use libsdp::SdpMediaFormat;
use libsdp::SdpAttribute;
use libsdp::SdpEncoding;
use libsdp::SdpConnection;
use libsdp::SdpMedia;

use gstreamer::ElementExtManual;
use gstreamer::ElementExt;
use gstreamer::GstObjectExt;

use nirah_core::prelude::*;

pub struct GSTreamerProvider {

}

impl GSTreamerProvider {

    pub fn new() -> NirahResult<GSTreamerProvider> {
        gstreamer::init().unwrap();
        Ok(GSTreamerProvider {})
    }

    pub async fn get_audio_source(&self, cfg: &mut ConfigFuture) -> NirahResult<String> {
        if let Some(conf_value) = cfg.get_config_value(&VariableKey::new("audio_input_device")).await? {
            Ok(format!("pulsesrc device={}", conf_value))
        } else {
            Ok("alsasrc".to_string())
        }
    }

    pub async fn get_audio_sink(&self, cfg: &mut ConfigFuture) -> NirahResult<String> {
        if let Some(conf_value) = cfg.get_config_value(&VariableKey::new("audio_output_device")).await? {
            Ok(format!("pulsesink device={}", conf_value))
        } else {
            Ok("alsasink".into())
        }
    }

    pub async fn get_udp_source(&self, local_port: u32) -> NirahResult<String> {
        Ok(format!("udpsrc caps=\"application/x-rtp\" port={}", local_port))
    }

    pub async fn get_udp_sink(&self, address: &str, port: u32) -> NirahResult<String> {
        Ok(format!("udpsink host={} port={}", address, port))
    }

    pub async fn get_input_encoders(&self, fmt: &SdpMediaFormat) -> NirahResult<String> {
        for attr in &fmt.attributes {
            if let SdpAttribute::RtpMap(map) = attr {
                match map.encoding {
                    SdpEncoding::Pcmu => return Ok("rtppcmudepay ! mulawdec".into()),
                    SdpEncoding::Pcma => return Ok("rtppcmadepay ! alawdec".into()),
                    _ => {}
                }
            }
        }
        Err(NirahError::InvalidMediaFormat(fmt.clone()))
    }

    pub async fn get_ouput_encoders(&self, fmt: &SdpMediaFormat) -> NirahResult<String> {
        for attr in &fmt.attributes {
            if let SdpAttribute::RtpMap(map) = attr {
                match map.encoding {
                    SdpEncoding::Pcmu => return Ok("mulawenc ! rtppcmupay".into()),
                    SdpEncoding::Pcma => return Ok("alawenc ! rtppcmapay".into()),
                    _ => {}
                }
            }
        }
        Err(NirahError::InvalidMediaFormat(fmt.clone()))
    }

    pub async fn create_input_pipeline<'a>(&self, ctx: &mut StreamingCtx<'a>, fmt: &SdpMediaFormat, local_port: u32) -> NirahResult<String> {

        Ok(format!(
            "{} ! queue ! {} ! queue ! {}",
            self.get_udp_source(local_port).await?,
            self.get_input_encoders(fmt).await?,
            self.get_audio_sink(&mut ctx.config).await?
        ))
    }

    pub async fn create_output_pipeline<'a>(&self, ctx: &mut StreamingCtx<'a>, fmt: &SdpMediaFormat, media: &SdpMedia, conn: Option<SdpConnection>) -> NirahResult<String> {
        let connection = if let Some(conn) = &fmt.connection {
            conn.clone()
        } else {
            if let Some(conn) = conn {
                conn
            } else {
                return Err(NirahError::SdpNoConnection)
            }
        };
        Ok(format!(
            "{} ! queue ! {} ! queue ! {}",
            self.get_audio_source(&mut ctx.config).await?,
            self.get_ouput_encoders(fmt).await?,
            self.get_udp_sink(&connection.address, media.port).await?
        ))
    }
}

impl Provider for GSTreamerProvider {
    fn nirah_provider_identifier(&self) -> &'static str {
        "GSTreamerStreamingProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn required_config_variables(&self) -> NirahResult<Vec<(VariableKey, Option<VariableValue>)>> {
        Ok(vec![
            (VariableKey::new("audio_output_device"), Some(VariableValue::String("alsa_output.usb-Logitech_G432_Gaming_Headset_000000000000-00.analog-stereo".into()))),
            (VariableKey::new("audio_input_device"), Some(VariableValue::String("alsa_output.pci-0000_00_1f.3.analog-stereo".into())))
        ])
    }
}

#[async_trait]
impl StreamingProvider for GSTreamerProvider {
    async fn list_streams(&self) -> NirahResult<Vec<String>> {
        Ok(vec![])
    }

    async fn handle_streams<'a>(&mut self, mut ctx: StreamingCtx<'a>, events: StreamingEvent) -> NirahResult<()> {
        println!("{:?}", events);
        let mut pipe_string = String::new();
        for input in &events.inputs {
            for media in &input.media {
                for format in &media.formats {
                    if pipe_string.len() > 0 {
                        pipe_string.push(' ');
                    }
                    pipe_string.push_str(&self.create_input_pipeline(&mut ctx, format, events.local_port).await?);
                }
            }
        }

        for output in &events.outputs {
            for media in &output.media {
                for format in &media.formats {
                    if pipe_string.len() > 0 {
                        pipe_string.push(' ');
                    }
                    pipe_string.push_str(
                        &self.create_output_pipeline(&mut ctx, format, media, output.get_connection()).await?
                    );
                }
            }
        }

        println!("{:?}", pipe_string);
        let pipeline =
            gstreamer::parse_launch_full(&pipe_string, None, gstreamer::ParseFlags::NONE)
            .expect("Failed to creatae pipeline");
        let bus = pipeline.get_bus().unwrap();
        pipeline
            .set_state(gstreamer::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");
        for msg in bus.iter_timed(gstreamer::CLOCK_TIME_NONE) {
            use gstreamer::MessageView;
            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                    break;
                }
                _ => (),
            }
        }
        pipeline
            .set_state(gstreamer::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");
        Ok(())
    }

    async fn end_stream<'a>(&mut self, _ctx: StreamingCtx<'a>, _call_id: String) -> NirahResult<()> {
        Ok(())
    }
}
