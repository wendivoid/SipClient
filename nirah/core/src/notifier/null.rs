use async_trait::async_trait;

use crate::prelude::*;

pub struct NullNotifierProvider;

impl Provider for NullNotifierProvider {

    fn nirah_provider_identifier(&self) -> &'static str {
        "NullNotifierProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[async_trait]
impl NotifierProvider for NullNotifierProvider {
    async fn new_transaction<'a>(&mut self, _: NotifierArgument<'a>) -> NirahResult<()> {
        Ok(())
    }
}
