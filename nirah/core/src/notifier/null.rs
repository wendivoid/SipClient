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

    fn required_config_variables(&self) -> NirahResult<Vec<(VariableKey, Option<VariableValue>)>> {
        Ok(vec![])
    }
}

#[async_trait]
impl NotifierProvider for NullNotifierProvider {
    async fn new_transaction<'a>(&mut self, _: NotifierArgument<'a>) -> NirahResult<()> {
        Ok(())
    }
}
