use serde_json::Error as JsonError;
use log::SetLoggerError;
use libsdp::SanitizerError;

use std::env::VarError;
use std::io::Error as IoError;
use std::string::FromUtf8Error;

use nom::Err;
use nom::error::ErrorKind;

use crate::config::VariableKey;

#[derive(Debug)]
pub enum NirahError {
    Io(IoError),
    FromUtf8(FromUtf8Error),
    Logger(SetLoggerError),
    Environment(VarError),
    InvalidConfigKey(VariableKey),
    InvalidAccountId(u32),
    Json(JsonError),
    SipParseError(String, ErrorKind),
    SipRegistrationFailed(u32),
    SdpSanitize(SanitizerError),
    InvalidContactId(u32),
    ParseIncomplete,
    SessionNotAssociatedWithAccount,
    NoNetworksAvailable,
}

macro_rules! impl_simple_from {
    ($ty:tt, $variant:tt) => {
        impl From<$ty> for NirahError {
            fn from(err: $ty) -> NirahError {
                NirahError::$variant(err)
            }
        }
    }
}

impl_simple_from!(IoError, Io);
impl_simple_from!(JsonError, Json);
impl_simple_from!(VarError, Environment);
impl_simple_from!(SetLoggerError, Logger);
impl_simple_from!(FromUtf8Error, FromUtf8);
impl_simple_from!(SanitizerError, SdpSanitize);

impl From<Err<(&[u8], ErrorKind)>> for NirahError {
    fn from(f: Err<(&[u8], ErrorKind)>) -> NirahError {
        match f {
            Err::Error((a, b)) => NirahError::SipParseError(String::from_utf8_lossy(a).into(), b),
            Err::Incomplete(_size) => NirahError::ParseIncomplete,
            Err::Failure((a, b)) => NirahError::SipParseError(String::from_utf8_lossy(a).into(), b)
        }
    }
}
