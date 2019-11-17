use clap::App;
use clap::Arg;
use clap::AppSettings;
use clap::ArgMatches;
use clap::SubCommand;
use ascii_table::Align;
use ascii_table::print_table;
use ascii_table::TableConfig;
use ascii_table::ColumnConfig;
use nirah_core::prelude::*;
use super::utils::print_response;
use super::utils::get_response;
use super::utils::OptionalDisplay;

pub fn args() -> App<'static, 'static> {
    SubCommand::with_name("contacts")
        .about("Manage Contacts")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("list")
                .about("List all contacts.")
        )
        .subcommand(SubCommand::with_name("get")
            .about("Get information about a contact.")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
                Arg::with_name("id")
                    .help("The id of the contact.")
                    .index(1)
                    .required(true)
                )
        )
        .subcommand(SubCommand::with_name("log")
             .about("Show the transaction log for a contact")
             .setting(AppSettings::ArgRequiredElseHelp)
             .arg(
                 Arg::with_name("id")
                     .help("The id of the contact")
                     .index(1)
                     .required(true)
             )
        )
        .subcommand(SubCommand::with_name("message")
             .about("Send a text message to this contact.")
             .setting(AppSettings::ArgRequiredElseHelp)
             .arg(
                 Arg::with_name("id")
                     .help("The id of the contact")
                     .index(1)
                     .required(true)
             )
             .arg(
                 Arg::with_name("account")
                     .help("The id of the account to send the message from.")
                     .index(2)
                     .required(true)
             )
             .arg(
                 Arg::with_name("message")
                     .help("Message data")
                     .index(3)
                     .required(true)
             )
        )
        .subcommand(SubCommand::with_name("create")
             .about("Create a new contact")
             .setting(AppSettings::ArgRequiredElseHelp)
             .arg(
                 Arg::with_name("uri")
                     .index(1)
                     .required(true)
                     .help("The uri of the desired contact")
             )
             .arg(
                 Arg::with_name("display_name")
                     .takes_value(true)
             )
        )
}

pub async fn handle(opt: Option<&ArgMatches<'static>>) -> NirahResult<()> {
        match opt.unwrap().subcommand() {
            ("get", Some(matches)) => {
                let id = value_t_or_exit!(matches, "id", u32);
                let req = RpcRequest::GetAccount { id };
                trace!("Request: {:?}", req);
                print_response(req).await
            },
            ("list", _) => {
                let response = get_response(RpcRequest::AllContacts).await?;
                match response {
                    RpcResponse::AllContacts { contacts } => {
                        let mut table_config = TableConfig::default();
                        table_config.columns.insert(0, ColumnConfig {
                            header: "Id".into(),
                            align: Align::Left
                        });
                        table_config.columns.insert(1, ColumnConfig {
                            header: "Display Name".into(),
                            align: Align::Left
                        });
                        table_config.columns.insert(2, ColumnConfig {
                            header: "Uri".into(),
                            align: Align::Left
                        });
                        let mut display_table = vec![];
                        for contact in contacts {
                            display_table.push(vec![
                                OptionalDisplay(Some(format!("{}", contact.id))),
                                OptionalDisplay(contact.display_name.clone()),
                                OptionalDisplay(Some(format!("{}", contact.uri)))
                            ]);
                        }
                        if display_table.len() == 0 {
                            display_table.push(vec![
                                OptionalDisplay(None),
                                OptionalDisplay(None),
                                OptionalDisplay(None)

                            ])
                        }
                        print_table(&display_table, &table_config);
                        Ok(())
                    },
                    _ => Ok(())
                }
            },
            ("log", Some(matches)) => {
                let contact = value_t_or_exit!(matches, "id", u32);
                let req = RpcRequest::ContactTransactions { contact };
                trace!("Request: {:?}", req);
                let response = get_response(req).await?;
                match response {
                    RpcResponse::ContactTransactions { transactions } => {
                        let mut table_config = TableConfig::default();
                        let mut display_table = vec![];
                        table_config.columns.insert(0, ColumnConfig {
                            header: "Account".into(),
                            align: ascii_table::Align::Left
                        });
                        table_config.columns.insert(1, ColumnConfig {
                            header: "Type".into(),
                            align: Align::Left
                        });
                        table_config.columns.insert(2, ColumnConfig {
                            header: "Time".into(),
                            align: Align::Left
                        });
                        table_config.columns.insert(3, ColumnConfig {
                            header: "Data".into(),
                            align: Align::Left
                        });
                        for transaction in transactions {
                            match transaction.data {
                                TransactionEventData::TextMessage { message } => {
                                    display_table.push(vec![
                                        OptionalDisplay(Some(format!("{}", transaction.account))),
                                        OptionalDisplay(Some("Message".into())),
                                        OptionalDisplay(Some(format!("{}",transaction.time.format("%d/%m/%Y %H:%S")))),
                                        OptionalDisplay(Some(message))
                                    ]);
                                },
                                TransactionEventData::Invitation { } => {

                                }
                            }
                        }
                        if display_table.len() == 0 {
                            display_table.push(vec![
                                OptionalDisplay(None),
                                OptionalDisplay(None),
                                OptionalDisplay(None),
                                OptionalDisplay(None),
                            ])
                        }
                        ascii_table::print_table(&display_table, &table_config);
                        Ok(())
                    },
                    _ => Ok(())
                }
            },
            ("message", Some(matches)) => {
                let contact = value_t_or_exit!(matches, "id", u32);
                let account = value_t_or_exit!(matches, "account", u32);
                let data = value_t_or_exit!(matches, "message", String);
                let transaction = NewTransactionEvent {
                    contact: contact,
                    account: account,
                    sent: true,
                    time: chrono::Utc::now().naive_utc(),
                    data: NewTransactionEventData::NewTextMessage {
                        message: data
                    }
                };
                let req = RpcRequest::PerformTransaction { account, contact, transaction };
                trace!("Request: {:?}", req);
                print_response(req).await
            },
            ("create", Some(matches)) => {
                let uri = value_t_or_exit!(matches, "uri", libsip::Uri);
                let display_name = matches.value_of("display_name").map(|item|item.to_string());
                let req = RpcRequest::CreateContact { contact: NewContact { uri, display_name }};
                print_response(req).await
            },
            _ => unreachable!()
        }
}
