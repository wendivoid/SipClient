#[macro_export]
macro_rules! context_config_get_string {
    ($ctx: ident, $k: tt) => {{
        let _key = $k.clone();
        match $ctx.config.get_config_value(&_key).await {
            Ok(Some(nirah_core::config::VariableValue::String(string))) => Ok(string.clone()),
            _ => Err(nirah_core::core::NirahError::InvalidConfigKey(_key.clone()))
        }};
    }
}

#[macro_export]
macro_rules! __context_config_get_string {
    ($ctx: ident, $k: tt) => {{
        let _key = $k.clone();
        match $ctx.config.get_config_value(&_key).await {
            Ok(Some(crate::config::VariableValue::String(string))) => Ok(string.clone()),
            _ => Err(crate::core::NirahError::InvalidConfigKey(_key.clone()))
        }};
    }
}

#[macro_export]
macro_rules! config_get_string {
    ($config: ident, $k: tt) => {{
        let _key = $k.clone();
        match $config.get_config_value(&_key).await {
            Ok(Some(nirah_core::config::VariableValue::String(string))) => Ok(string.clone()),
            _ => Err(nirah_core::core::NirahError::InvalidConfigKey(_key.clone()))
        }};
    }
}

#[macro_export]
macro_rules! __config_get_string {
    ($config: ident, $k: tt) => {{
        let _key = $k.clone();
        match $config.get_config_value(&_key).await {
            Ok(Some(crate::config::VariableValue::String(string))) => Ok(string.clone()),
            _ => Err(crate::core::NirahError::InvalidConfigKey(_key.clone()))
        }};
    }
}
