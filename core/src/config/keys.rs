use super::VariableKey;
use super::VariableValue;

pub fn default_ip_interface() -> VariableKey {
    VariableKey::new("default_ip_interface")
}

pub fn default_ip_interface_value() -> VariableValue {
    VariableValue::String("wlp2s0".into())
}
