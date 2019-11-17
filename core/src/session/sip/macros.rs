macro_rules! unwrap_or_else_not_connected {
    ($self:tt, $field: tt, $msg: tt) => {
        if let Some(key) = &$self.$field {
            key
        } else {
            return crate::session::sip::errors::not_connected($msg);
        }
    }
}

macro_rules! unwrap_mut_or_else_not_connected {
    ($self:tt, $field: tt, $msg: tt) => {
        if let Some(key) = &mut $self.$field {
            key
        } else {
            return crate::session::sip::errors::not_connected($msg);
        }
    }
}
