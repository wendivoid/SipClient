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
    SubCommand::with_name("streaming")
        .about("Manage Streaming Sessions")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .about("List all Active streaming sessions.")
        )
}

pub async fn handle(opt: Option<&ArgMatches<'static>>) -> NirahResult<()> {
        match opt.unwrap().subcommand() {
            ("list", _) => handle_list().await,
            _ => unreachable!()
        }
}

async fn handle_list() -> NirahResult<()> {
    let response = get_response(RpcRequest::AllCurrentStreams).await?;
    let mut table_config = TableConfig::default();
    table_config.width = 150;
    let mut display_table = vec![];
    table_config.columns.insert(0, ColumnConfig {
        header: "Name".into(),
        align: Align::Left
    });
    if let RpcResponse::AllStreams { streams } = response {
        if streams.len() == 0 {
            display_table.push(vec![OptionalDisplay(None)]);
        } else {
            for stream in streams {
                display_table.push(vec![
                    OptionalDisplay(Some(stream))
                ]);
                }
        }
    }
    print_table(&display_table, &table_config);
    Ok(())
}
