use serde::{ Serialize, Deserialize };

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "response")]
pub enum RpcResponse {
    /// Error response's
    AccountSessionNotActive,
    InvalidAccount { id: u32 },
    InvalidContact { id: u32 },
    InvalidConfigKey { key: VariableKey },
    /// Success response's
    Ok,
    Account { acc: Account },
    AllAccounts { accounts: Vec<Account> },
    Config { value: VariableValue },
    AllConfigVariables { vars: Vec<(VariableKey, Option<VariableValue>, Option<VariableValue>)> },
    Contact { contact: Contact },
    AllContacts { contacts: Vec<Contact> },
    ContactTransactions { transactions: Vec<TransactionEvent> },
    AboutNirah {
        accounts: (String, String),
        config: (String, String),
        contacts: (String, String),
        database: (String, String),
        notifier: (String, String),
        rpc: (String, String),
        rpc_handler: (String, String),
        sessions: Vec<(String, String)>
    }
}
