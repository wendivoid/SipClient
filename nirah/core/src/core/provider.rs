use super::NirahResult;
use crate::config::VariableKey;
use crate::config::VariableValue;

pub trait Provider: Send + Sync {

    fn nirah_provider_identifier(&self) -> &'static str;

    fn nirah_provider_version(&self) -> &'static str;

    fn required_config_variables(&self) -> NirahResult<Vec<(VariableKey, Option<VariableValue>)>>;
}
