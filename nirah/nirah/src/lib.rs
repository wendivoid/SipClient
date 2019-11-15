#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate nirah_core;
extern crate ascii_table;

use tokio::net::UnixStream;

use nirah_core::core::NirahResult;
use nirah_core::server::Server;
use nirah_core::server::Builder;

pub mod uds;
pub mod cli;
mod audio;
pub use self::audio::RodioAudioProvider;

pub async fn create_server() -> NirahResult<Server<UnixStream>> {
    Ok(
        Builder::new()
         .audio(Box::new(RodioAudioProvider::new()))
         .rpc(Box::new(uds::UdsRpcProvider::new()))
         .build()?
     )
}

pub fn set_log_level(n: u64) -> NirahResult<()> {
    use log::Level;
    use simple_logger::init_with_level as init;
    let lvl = match n {
        0 => Level::Warn,
        1 => Level::Info,
        2 => Level::Debug,
        _ => Level::Trace
    };
    Ok(init(lvl)?)
}
