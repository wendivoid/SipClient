use async_trait::async_trait;

use crate::core::Provider;
use crate::core::NirahResult;
use crate::core::NirahError;
use crate::config::VariableKey;
use crate::config::VariableValue;
use crate::config::ConfigDefinition;
use crate::config::ConfigSetting;

use std::collections::HashMap;

use super::ConfigProvider;

pub struct InMemoryConfigProvider(HashMap<VariableKey, ConfigSetting>);

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
    async fn register_config_setting(&mut self, setting: &ConfigDefinition) -> NirahResult<()> {
        if let Some(_) = self.0.get(&setting.key) {
            warn!("Tried to register config variable `{:?}` that was already registered.", setting.key);
            Err(NirahError::InvalidConfigKey(setting.key.clone()))
        } else {
            debug!("Registering config setting key: `{:?}`, default: `{:?}`", &setting.key, &setting.default);
            self.0.insert(setting.key.clone(), ConfigSetting {
                key: setting.key.clone(),
                default: setting.default.clone(),
                value: None,
                description: setting.description.clone()
            });
            Ok(())
        }
    }

    async fn register_config_settings(&mut self, settings: &[ConfigDefinition]) -> NirahResult<()> {
        for setting in settings {
            self.register_config_setting(setting).await?;
        }
        Ok(())
    }

    async fn get_config_value(&self, key: &VariableKey) -> NirahResult<Option<VariableValue>> {
        if let Some(conf) = self.0.get(key) {
            if conf.value.is_some() {
                Ok(conf.value.clone())
            } else {
                Ok(conf.default.clone())
            }
        } else {
            Err(NirahError::InvalidConfigKey(key.clone()))
        }
    }

    async fn set_config_value(&mut self, key: &VariableKey, value: Option<VariableValue>) -> NirahResult<()> {
        if let Some(conf) = self.0.get_mut(&key) {
            conf.value = value;
            Ok(())
        } else {
            Err(NirahError::InvalidConfigKey(key.clone()))
        }
    }

    async fn all_config_variables(&self) -> Vec<ConfigSetting> {
        self.0.values().map(|item|item.clone()).collect()
    }
}
