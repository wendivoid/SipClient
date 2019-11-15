use serde_json::Error as JsonError;
use log::SetLoggerError;
#[cfg(feature = "glib")]
use glib::error::Error as GlibError;
#[cfg(feature = "glib")]
use glib::error::BoolError as GlibBoolError;
#[cfg(feature = "gstreamer")]
use gstreamer::StateChangeError;

use std::env::VarError;
use std::io::Error as IoError;
use std::string::FromUtf8Error;

use nom::Err;
use nom::error::ErrorKind;

use crate::streaming::StreamingError;
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
    InvalidContactId(u32),
    ParseIncomplete,
    SessionNotAssociatedWithAccount,
    NoNetworksAvailable,
    Streaming(StreamingError),
    #[cfg(feature = "glib")]
    Glib(GlibError),
    #[cfg(feature = "glib")]
    GBool(GlibBoolError),
    #[cfg(feature = "gstreamer")]
    GStateChange(StateChangeError)
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

impl_simple_from!(StreamingError, Streaming);
impl_simple_from!(IoError, Io);
impl_simple_from!(JsonError, Json);
impl_simple_from!(VarError, Environment);
impl_simple_from!(SetLoggerError, Logger);
impl_simple_from!(FromUtf8Error, FromUtf8);
#[cfg(feature = "glib")]
impl From<GlibError> for NirahError {
    fn from(err: GlibError) -> NirahError {
        NirahError::Glib(err)
    }
}
#[cfg(feature = "glib")]
impl From<GlibBoolError> for NirahError {
    fn from(err: GlibBoolError) -> NirahError {
        NirahError::GBool(err)
    }
}
#[cfg(feature = "gstreamer")]
impl From<StateChangeError> for NirahError {
    fn from(err: StateChangeError) -> NirahError {
        NirahError::GStateChange(err)
    }
}

impl From<Err<(&[u8], ErrorKind)>> for NirahError {
    fn from(f: Err<(&[u8], ErrorKind)>) -> NirahError {
        match f {
            Err::Error((a, b)) => NirahError::SipParseError(String::from_utf8_lossy(a).into(), b),
            Err::Incomplete(_size) => NirahError::ParseIncomplete,
            Err::Failure((a, b)) => NirahError::SipParseError(String::from_utf8_lossy(a).into(), b)
        }
    }
}
