use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionEventData {
    TextMessage {
        message: String,
    },
    Invitation {

    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NewTransactionEventData {
    NewTextMessage {
        message: String,
    },
    NewInvitation {

    }
}
