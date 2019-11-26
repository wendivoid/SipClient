use async_trait::async_trait;

use crate::core::NirahResult;
use crate::accounts::Account;
use crate::contacts::Contact;
use crate::database::TransactionEvent;
use crate::core::Provider;

mod null;
pub use self::null::NullNotifierProvider;

pub struct NotifierParams {
    pub invite_id: Option<usize>
}

pub struct NotifierArgument<'a> {
    pub account: &'a Account,
    pub contact: &'a Contact,
    pub params: NotifierParams,
    pub msg: &'a TransactionEvent
}

#[async_trait]
pub trait NotifierProvider: Provider {
    async fn new_transaction<'a>(&mut self, arg: NotifierArgument<'a>) -> NirahResult<()>;
}
