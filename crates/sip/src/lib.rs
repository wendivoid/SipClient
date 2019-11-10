//! nirah-sip has two basic components a parser and managers.
//!
//! Managers are utility struct's meant to ease one specifc asspect of the sip protocol,
//! the only manager currently implemented is for endpoint registration.
//!
//! ### Parsing
//! nirah-sip expose- many parsing function though only one `parse_message` is needed.
//! ```rust
//!   extern crate nirah_sip;
//!
//!   use nirah_sip::parse_message;
//!
//!   let packet = "SIP/2.0 200 OK\r\n\r\n";
//!   let output = nirah_sip::parse_message(packet.as_ref()).unwrap();
//! ```
//!
//! ### Creating Messages
//! This crate provides 2 abstraction's to aid in building sip messages.
//! The `ResponseGenerator` is used to create sip response's and the
//! `RequestGenerator` is used to generate sip requests.
//!  ```rust
//!     extern crate nirah_sip;
//!     extern crate nirah_uri;
//!
//!     use nirah_sip::ResponseGenerator;
//!     use nirah_sip::RequestGenerator;
//!     use nirah_sip::Method;
//!     use nirah_uri::parse_uri;
//!
//!     let _res = ResponseGenerator::new()
//!                         .code(200)
//!                         .build()
//!                         .unwrap();
//!
//!     let uri = parse_uri("sip:1@0.0.0.0:5060;transport=UDP".as_ref()).unwrap().1;
//!     let _req = RequestGenerator::new()
//!                         .method(Method::Invite)
//!                         .uri(uri)
//!                         .build()
//!                         .unwrap();
//!  ```
//!
//! ### Registration
//! The registration manager is used to generate REGISTER requests. Once
//! that is sent to the server you must wait for the Challange response pass
//! it to the `set_challenge` method of the RegistrationManager.
//! reqpeatedly calling the `get_request` method will cause the c_nonce
//! counter to be incremented and a new hash computed.

#[macro_use]
extern crate nom;
extern crate serde;

#[macro_use]
mod macros;

pub mod core;
pub mod headers;
pub mod response;
pub mod request;
pub mod client;


pub use crate::core::Version;
pub use crate::core::Method;
pub use crate::headers::Header;
pub use crate::headers::Headers;

pub use crate::core::SipMessage;
pub use crate::core::message::parse_message;
pub use crate::response::ResponseGenerator;
pub use crate::request::RequestGenerator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
