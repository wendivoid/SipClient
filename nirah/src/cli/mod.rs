use clap::ArgMatches;
use clap::App;
use clap::Arg;
use clap::AppSettings;
use clap::SubCommand;
use ascii_table::ColumnConfig;
use ascii_table::TableConfig;
use ascii_table::Align;

use nirah_core::core::NirahResult;
use nirah_core::rpc::RpcResponse;

mod accounts;
mod config;
mod utils;
mod contacts;

pub fn get_args() -> ArgMatches<'static> {
    App::new("nirahctl")
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author(crate_authors!())
        .subcommand(config::args())
        .subcommand(accounts::args())
        .subcommand(contacts::args())
        .subcommand(SubCommand::with_name("about"))
        .arg(Arg::with_name("json")
            .short("j")
            .long("json")
            .help("Output raw JSON")
        )
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity")
        )
        .get_matches()
}

pub async fn run() -> NirahResult<()> {
    let args = get_args();
    crate::set_log_level(args.occurrences_of("verbose"))?;
    let json_output = args.is_present("json");
    match args.subcommand() {
        ("about", _) => handle_about(json_output).await,
        ("config", matches) => config::handle(matches, json_output).await,
        ("accounts", matches) => accounts::handle(matches, json_output).await,
        ("contacts", matches) => contacts::handle(matches, json_output).await,
        _ => unreachable!()
    }
}

async fn handle_about(json_output: bool) -> NirahResult<()> {
    let req = nirah_core::rpc::RpcRequest::AboutNirah;
    let response = utils::get_response(req).await?;
    if json_output {
        println!("{}", serde_json::to_string(&response)?);
        return Ok(());
    }
    let mut table_config = TableConfig::default();
    let mut display_table = vec![];
    table_config.columns.insert(0, ColumnConfig {
        header: "Role".into(),
        align: ascii_table::Align::Left
    });
    table_config.columns.insert(1, ColumnConfig {
        header: "Identifier".into(),
        align: Align::Left
    });
    table_config.columns.insert(2, ColumnConfig {
        header: "Version".into(),
        align: Align::Left
    });
    if let RpcResponse::AboutNirah {
        accounts, config,
        contacts, database, notifier,
        rpc, rpc_handler, sessions
    } = response {
        display_table.push(vec!["accounts".to_string(), accounts.0, accounts.1]);
        display_table.push(vec!["config".to_string(), config.0, config.1]);
        display_table.push(vec!["contacts".to_string(), contacts.0, contacts.1]);
        display_table.push(vec!["database".to_string(), database.0, database.1]);
        display_table.push(vec!["notifier".to_string(), notifier.0, notifier.1]);
        display_table.push(vec!["rpc".to_string(), rpc.0, rpc.1]);
        display_table.push(vec!["rpc_handler".to_string(), rpc_handler.0, rpc_handler.1]);
        for sess in sessions {
            display_table.push(vec!["session".to_string().into(), sess.0, sess.1]);
        }
    }
    ascii_table::print_table(display_table, &table_config);
    Ok(())
}
