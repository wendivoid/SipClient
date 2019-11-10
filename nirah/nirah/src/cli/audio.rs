use clap::App;
use clap::AppSettings;
use clap::ArgMatches;
use clap::SubCommand;
use nirah_core::core::NirahResult;


pub fn args() -> App<'static, 'static> {
    SubCommand::with_name("audio")
        .about("Manage Audio")
        .setting(AppSettings::SubcommandRequiredElseHelp)

}

pub async fn handle(opt: Option<&ArgMatches<'static>>) -> NirahResult<()> {
        match opt.unwrap().subcommand() {
            _ => unreachable!()
        }
}
