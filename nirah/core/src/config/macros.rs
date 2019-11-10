macro_rules! context_config_get_string {
    ($ctx: ident, $k: tt, $d: tt) => {{
        let _key = $k.clone();
        match $ctx.config.get_config_value(&_key).await?.unwrap_or($d.clone()) {
            crate::config::VariableValue::String(string) => Ok(string.clone()),
            _ => Err(crate::core::NirahError::InvalidConfigKey(_key.clone()))
        }};
    }
}
