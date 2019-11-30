use clap::App;
use clap::Arg;
use clap::AppSettings;
use clap::ArgMatches;
use clap::SubCommand;
use nirah_core::core::NirahResult;
use nirah_core::accounts::NewAccount;
use nirah_core::accounts::AccountType;
use nirah_core::rpc::RpcRequest;
use nirah_core::rpc::RpcResponse;
use ascii_table::TableConfig;
use ascii_table::ColumnConfig;
use ascii_table::print_table;
use ascii_table::Align;

use std::collections::HashMap;

use super::utils::print_response;
use super::utils::get_response;
use super::utils::OptionalDisplay;

pub fn args() -> App<'static, 'static> {
    SubCommand::with_name("accounts")
        .about("Manage Accounts")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .about("List all accounts.")
        )
        .subcommand(SubCommand::with_name("init")
                .about("Initialize an account")
                .arg(
                    Arg::with_name("id")
                        .help("The account id")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(SubCommand::with_name("get")
            .about("Get information about an account.")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
                Arg::with_name("index")
                    .help("The index of the account.")
                    .index(1)
                    .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove an account.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("id")
                        .index(1)
                        .help("The id of the account to remove")
                        .required(true)
                        .takes_value(true)
                    )
        )
        .subcommand(SubCommand::with_name("create")
            .about("Add a new account")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
                Arg::with_name("activate")
                    .help("Will this account connect automatically")
                    .short("a")
                    .long("activate")
                    .required(false)
            )
            .arg(
                Arg::with_name("user")
                    .help("The username to use when authenticating this account.")
                    .short("u")
                    .required(true)
                    .long("user")
                    .takes_value(true)
                )
            .arg(
                Arg::with_name("pass")
                    .help("The password to use when authenticating this account.")
                    .short("p")
                    .required(true)
                    .long("pass")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("server")
                    .help("Server address of the message account.")
                    .short("s")
                    .required(true)
                    .long("server")
                    .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("accept")
        .about("Accept an invitation request")
        .arg(
            Arg::with_name("account")
            .help("The id of the account the invitation is for")
            .index(1)
            .required(true)
        )
        .arg(
            Arg::with_name("invite")
            .help("The id of the invitation to accept")
            .index(2)
            .required(true)
        ))
        .subcommand(SubCommand::with_name("end-call")
        .about("End a currently ongoing call")
        .arg(
            Arg::with_name("account")
            .help("The id of the account the invitation is for")
            .index(1)
            .required(true)
        )
        .arg(
            Arg::with_name("call")
            .help("The call id to end")
            .index(2)
            .required(true)
        )
    )
}

pub async fn handle(opt: Option<&ArgMatches<'static>>, json_output: bool) -> NirahResult<()> {
        match opt.unwrap().subcommand() {
            ("list", _) => {
                let response = get_response(RpcRequest::AllAccounts).await?;
                if json_output {
                    println!("{}", serde_json::to_string(&response)?);
                    return Ok(());
                }
                match response {
                    RpcResponse::AllAccounts { accounts } => {
                        let mut table_config = TableConfig::default();
                        let mut display_table = vec![];
                        table_config.columns.insert(0, ColumnConfig {
                            header: "Id".into(),
                            align: Align::Left
                        });
                        table_config.columns.insert(1, ColumnConfig {
                            header: "Type".into(),
                            align: Align::Left
                        });
                        table_config.columns.insert(2, ColumnConfig {
                            header: "Username".into(),
                            align: Align::Left
                        });
                        table_config.columns.insert(3, ColumnConfig {
                            header: "Password".into(),
                            align: Align::Left
                        });
                        table_config.columns.insert(4, ColumnConfig {
                            header: "Host".into(),
                            align: Align::Left
                        });
                        table_config.columns.insert(5, ColumnConfig {
                            header: "Vars".into(),
                            align: Align::Left
                        });
                        for account in accounts {
                            display_table.push(vec![
                                OptionalDisplay(Some(format!("{}", account.id))),
                                OptionalDisplay(Some(format!("{:?}", account.ty))),
                                OptionalDisplay(Some(account.username)),
                                OptionalDisplay(Some(account.password)),
                                OptionalDisplay(Some(account.host)),
                                OptionalDisplay(Some(format!("{:?}", account.vars)))
                            ]);
                        }
                        if display_table.len() == 0 {
                            display_table.push(vec![
                                OptionalDisplay(None),
                                OptionalDisplay(None),
                                OptionalDisplay(None),
                                OptionalDisplay(None),
                                OptionalDisplay(None),
                                OptionalDisplay(None),

                            ])
                        }
                        print_table(&display_table, &table_config);
                        Ok(())
                    },
                    _ => Ok(())
                }
            },
            ("init", Some(matches)) => {
                let id = value_t_or_exit!(matches, "id", u32);
                let req = RpcRequest::InitializeAccount { id };
                print_response(req, json_output).await
            },
            ("get", Some(matches)) => {
                let dex = value_t_or_exit!(matches, "index", u32);
                let req = RpcRequest::GetAccount { id: dex };
                trace!("Request: {:?}", req);
                print_response(req, json_output).await
            },
            ("create", Some(matches)) => {
                let new = NewAccount {
                    ty: AccountType::Sip,
                    username: value_t_or_exit!(matches, "user", String),
                    password: value_t_or_exit!(matches, "pass", String),
                    host: value_t_or_exit!(matches, "server", String),
                    vars: HashMap::new()
                };
                let req = RpcRequest::CreateAccount { new };
                trace!("Request: {:?}", req);
                print_response(req, json_output).await
            },
            ("remove", Some(matches)) => {
                let id = value_t_or_exit!(matches, "id", u32);
                let req = RpcRequest::RemoveAccount { id };
                trace!("Request: {:?}", req);
                print_response(req, json_output).await
            },
            ("accept", Some(matches)) => {
                let account = value_t_or_exit!(matches, "account", u32);
                let invite = value_t_or_exit!(matches, "invite", usize);
                let req = RpcRequest::AcceptInvite { account, invite };
                trace!("Request: {:?}", req);
                print_response(req, json_output).await
            },
            ("end-call", Some(matches)) => {
                let account = value_t_or_exit!(matches, "account", u32);
                let call = value_t_or_exit!(matches, "call", String);
                let req = RpcRequest::EndCall { account, call };
                trace!("Request: {:?}", req);
                print_response(req, json_output).await
            },
            _ => unreachable!()
        }
}
