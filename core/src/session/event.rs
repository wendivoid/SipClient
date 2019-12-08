use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum SessionEvent {
    Timeout,
    AcceptInvite { invite: usize },
    Bye { call: String },
    Transaction { transaction: TransactionEvent },
    Data { data: Vec<u8> }
}
