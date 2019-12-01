use async_trait::async_trait;
use serde::{ Serialize, Deserialize };

use crate::core::Provider;
use crate::core::NirahResult;

pub mod keys;
mod var_key;
pub use self::var_key::VariableKey;

mod var_value;
pub use self::var_value::VariableValue;

mod memory;
pub use self::memory::InMemoryConfigProvider;

#[macro_use]
mod macros;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConfigDefinition {
    pub key: VariableKey,
    pub default: Option<VariableValue>,
    pub description: Option<String>
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConfigSetting {
    pub key: VariableKey,
    pub default: Option<VariableValue>,
    pub value: Option<VariableValue>,
    pub description: Option<String>
}

#[async_trait]
pub trait ConfigProvider: Provider {

    async fn register_config_setting(&mut self, setting: &ConfigDefinition) -> NirahResult<()>;

    async fn register_config_settings(&mut self, settings: &[ConfigDefinition]) -> NirahResult<()>;

    async fn get_config_value(&self, key: &VariableKey) -> NirahResult<Option<VariableValue>>;

    async fn set_config_value(&mut self, key: &VariableKey, value: Option<VariableValue>) -> NirahResult<()>;

    async fn all_config_variables(&self) -> Vec<ConfigSetting>;
}
