/// Generate a URI domain from an domain name.
/// ```rust
///    #[macro_use] extern crate nirah_uri;
///
///    let domain = domain!("example.com");
///    let domain = domain!("example.com", 5060);
/// ```
#[macro_export]
macro_rules! domain {
    ($domain:tt) => {
        nirah_uri::Domain::Domain($domain.into(), None)
    };
    ($domain:tt, $port:tt) => {
        nirah_uri::Domain::Domain($domain.into(), Some($port))
    };
}

/// Generate a URI domain from an ip address.
/// ```rust
///    #[macro_use] extern crate nirah_uri;
///
///    let domain = ip_domain!(192, 168, 0, 1);
///    let domain = ip_domain!(192, 168, 0, 1, 5060);
/// ```
#[macro_export]
macro_rules! ip_domain {
    ($a:tt, $b:tt, $c:tt, $d:tt) => {
        nirah_uri::Domain::Ipv4(::std::net::Ipv4Addr::new($a, $b, $c, $d), None)
    };

    ($a:tt, $b:tt, $c:tt, $d:tt, $port:tt) => {
        nirah_uri::Domain::Ipv4(::std::net::Ipv4Addr::new($a, $b, $c, $d), Some($port))
    };
}

/// Generate a URI authentication from credentials.
/// ```rust
///    #[macro_use] extern crate nirah_uri;
///
///    let auth = uri_auth!("user");
///    let auth = uri_auth!("user", "pass");
/// ```
#[macro_export]
macro_rules! uri_auth {
    ($u: tt) => {
        nirah_uri::UriAuth::new($u)
    };
    ($u: tt, $p: tt) => {
        nirah_uri::UriAuth::new($u).password($p)
    }
}
