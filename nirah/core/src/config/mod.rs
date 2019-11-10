use async_trait::async_trait;

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

#[async_trait]
pub trait ConfigProvider: Provider {

    async fn register_config_setting(&mut self, key: VariableKey, default: Option<VariableValue>) -> NirahResult<()>;

    async fn register_config_settings(&mut self, settings: &[(VariableKey, Option<VariableValue>)]) -> NirahResult<()>;

    async fn get_config_value(&self, key: &VariableKey) -> NirahResult<Option<VariableValue>>;

    async fn set_config_value(&mut self, key: &VariableKey, value: VariableValue) -> NirahResult<()>;

    async fn all_config_variables(&self) -> Vec<(VariableKey, Option<VariableValue>, Option<VariableValue>)>;
}
