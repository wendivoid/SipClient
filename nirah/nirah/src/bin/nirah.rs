#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

extern crate nirah_core;
extern crate nirah;

use clap::App;
use clap::ArgMatches;
use clap::Arg;

use nirah_core::core::NirahResult;

fn args() -> ArgMatches<'static> {
    App::new(crate_name!())
      .version(crate_version!())
      .about(crate_description!())
      .author(crate_authors!())
      .arg(Arg::with_name("verbose")
          .short("v")
          .multiple(true)
          .help("Sets the level of verbosity")
      )
      .get_matches()
}

#[tokio::main]
async fn main() -> NirahResult<()> {
    let args = args();
    nirah::set_log_level(args.occurrences_of("verbose"))?;
    let server = nirah::create_server().await?;
    if let Err(err) = server.mainloop().await {
        error!("{:?}", err);
    }
    Ok(())
}
