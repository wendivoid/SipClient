use async_trait::async_trait;

use crate::core::Provider;
use crate::core::NirahResult;
use crate::core::NirahError;
use crate::config::VariableKey;
use crate::config::VariableValue;

use std::collections::HashMap;

use super::ConfigProvider;

pub struct InMemoryConfigProvider(HashMap<VariableKey, (Option<VariableValue>, Option<VariableValue>)>);

impl InMemoryConfigProvider {

    pub fn new() -> InMemoryConfigProvider {
        InMemoryConfigProvider(HashMap::new())
    }
}

impl Provider for InMemoryConfigProvider {

    fn nirah_provider_identifier(&self) -> &'static str {
        "InMemoryConfigProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[async_trait]
impl ConfigProvider for InMemoryConfigProvider {
    async fn register_config_setting(&mut self, key: VariableKey, default: Option<VariableValue>) -> NirahResult<()> {
        if let Some(_) = self.0.get(&key) {
            warn!("Tried to register config variable `{:?}` that was already registered.", key);
            Err(NirahError::InvalidConfigKey(key.clone()))
        } else {
            debug!("Registering config setting key: `{:?}`, default: `{:?}`", key, default);
            self.0.insert(key.clone(), (default, None));
            Ok(())
        }
    }

    async fn register_config_settings(&mut self, settings: &[(VariableKey, Option<VariableValue>)]) -> NirahResult<()> {
        for (key, value) in settings {
            self.register_config_setting(key.clone(), value.clone()).await?;
        }
        Ok(())
    }

    async fn get_config_value(&self, key: &VariableKey) -> NirahResult<Option<VariableValue>> {
        if let Some((default, value)) = self.0.get(key) {
            if value.is_some() {
                Ok(value.clone())
            } else {
                Ok(default.clone())
            }
        } else {
            Err(NirahError::InvalidConfigKey(key.clone()))
        }
    }

    async fn set_config_value(&mut self, key: &VariableKey, value: VariableValue) -> NirahResult<()> {
        if let Some((_, old_value)) = self.0.get_mut(&key) {
            *old_value = Some(value);
            Ok(())
        } else {
            Err(NirahError::InvalidConfigKey(key.clone()))
        }
    }

    async fn all_config_variables(&self) -> Vec<(VariableKey, Option<VariableValue>, Option<VariableValue>)> {
        self.0.iter().map(|(a, (b, c))| (a.clone(), b.clone(), c.clone())).collect()
    }
}
