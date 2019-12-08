use crate::accounts::Account;
use crate::contacts::Contact;
use crate::database::TransactionEvent;

use super::NotifierParams;

pub struct NotifierArgument<'a> {
    pub account: &'a Account,
    pub contact: &'a Contact,
    pub params: NotifierParams,
    pub msg: &'a TransactionEvent
}
