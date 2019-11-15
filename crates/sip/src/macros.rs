/// Generate `NamedHeader` from a uri;
/// ```rust
///    #[macro_use] extern crate nirah_uri;
///    #[macro_use] extern crate nirah_sip;
///
///    let uri = nirah_uri::Uri::sip(domain!("example.com"));
///    let domain = named_header!(uri);
/// ```
#[macro_export]
macro_rules! named_header {
    ($u:tt) => {
        nirah_sip::headers::NamedHeader { display_name: None, uri: $u, params: ::std::collections::HashMap::new() }
    }
}
