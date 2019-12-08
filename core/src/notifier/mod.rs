use async_trait::async_trait;

use crate::core::NirahResult;
use crate::core::Provider;

mod null;
pub use self::null::NullNotifierProvider;

mod argument;
pub use self::argument::NotifierArgument;

mod params;
pub use self::params::NotifierParams;

#[async_trait]
pub trait NotifierProvider: Provider {
    async fn new_transaction<'a>(&mut self, arg: NotifierArgument<'a>) -> NirahResult<()>;
}
