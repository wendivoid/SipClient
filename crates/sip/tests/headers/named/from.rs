use nirah_uri::Uri;
use nirah_sip::headers::parse::parse_from_header;
use nirah_sip::headers::Header;
use nirah_sip::headers::NamedHeader;

#[test]
fn write() {
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(NamedHeader::new(uri).name("Guy"));
    assert_eq!("From: Guy <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(NamedHeader::new(uri).name("Guy With Face"));
    assert_eq!("From: \"Guy With Face\" <sip:guy@example.com>".to_string(), format!("{}", header));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(NamedHeader::new(uri));
    assert_eq!("From: <sip:guy@example.com>".to_string(), format!("{}", header));
}

#[test]
fn read() {
    let remains = vec![];
    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(NamedHeader::new(uri).name("Guy"));
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: Guy <sip:guy@example.com>\r\n"));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(NamedHeader::new(uri).name("Guy with face"));
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: \"Guy with face\" <sip:guy@example.com>\r\n"));

    let uri = Uri::sip(domain!("example.com")).auth(uri_auth!("guy"));
    let header = Header::From(NamedHeader::new(uri));
    assert_eq!(Ok((remains.as_ref(), header)), parse_from_header(b"From: <sip:guy@example.com>\r\n"));
}
