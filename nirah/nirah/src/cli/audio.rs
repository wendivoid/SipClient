use clap::App;
use clap::AppSettings;
use clap::ArgMatches;
use clap::SubCommand;
use nirah_core::core::NirahResult;
use ascii_table::TableConfig;
use ascii_table::ColumnConfig;
use ascii_table::print_table;
use ascii_table::Align;
use nirah_core::rpc::RpcResponse;
use nirah_core::rpc::RpcRequest;

use super::utils::OptionalDisplay;
use super::utils::get_response;

pub fn args() -> App<'static, 'static> {
    SubCommand::with_name("audio")
        .about("Manage Audio")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .about("List all audio devices.")
        )
}

pub async fn handle(opt: Option<&ArgMatches<'static>>) -> NirahResult<()> {
        match opt.unwrap().subcommand() {
            ("list", _) => handle_list().await,
            _ => unreachable!()
        }
}

async fn handle_list() -> NirahResult<()> {
    let response = get_response(RpcRequest::AllAudioDevices).await?;
    let mut table_config = TableConfig::default();
    table_config.width = 150;
    let mut display_table = vec![];
    table_config.columns.insert(0, ColumnConfig {
        header: "Direction".into(),
        align: Align::Left
    });
    table_config.columns.insert(1, ColumnConfig {
        header: "Name".into(),
        align: Align::Left
    });
    table_config.columns.insert(2, ColumnConfig {
        header: "Description".into(),
        align: Align::Left
    });
    if let RpcResponse::AudioDevices { devices } = response {
        for device in devices {
            display_table.push(vec![
                OptionalDisplay(Some(format!("{}", device.direction))),
                OptionalDisplay(device.name),
                OptionalDisplay(device.description.map(|item|format!("{:?}", item)))
            ]);
        }
    }
    print_table(&display_table, &table_config);
    Ok(())
}
