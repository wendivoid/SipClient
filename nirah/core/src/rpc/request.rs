use serde::{ Serialize, Deserialize };

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum RpcRequest {
    /// Configuration
    AboutNirah,
    AllVariables,
    GetConfig { key: VariableKey },
    SetConfig { key: VariableKey, value: VariableValue },

    /// Accounts
    AllAccounts,
    GetAccount { id: u32 },
    CreateAccount { new: NewAccount },
    EditAccount { account: Account },
    RemoveAccount { id: u32 },
    InitializeAccount { id: u32 },
    AcceptInvite { account: u32, invite: usize },

    /// Contacts
    AllContacts,
    GetContact { id: u32 },
    CreateContact { contact: NewContact },
    EditContact { contact: Contact },
    RemoveContact { id: u32 },
    ContactTransactions { contact: u32 },
    PerformTransaction { account: u32, contact: u32, transaction: NewTransactionEvent }
}
