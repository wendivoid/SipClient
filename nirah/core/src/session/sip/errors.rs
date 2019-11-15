use crate::prelude::*;
use std::io;

pub(crate) fn not_connected<T>(msg: &'static str) -> NirahResult<T> {
    Err(io::Error::new(io::ErrorKind::NotConnected, msg).into())
}
