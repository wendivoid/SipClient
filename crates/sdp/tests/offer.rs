use nirah_sdp::*;

#[test]
fn wikipedia_example() {
    let data = "v=0\r
o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5\r
s=SDP Seminar\r
i=A Seminar on the session description protocol\r
u=http://www.example.com/seminars/sdp.pdf\r
e=j.doe@example.com (Jane Doe)\r
c=IN IP4 224.2.17.12/127\r
t=2873397496 2873404696\r
a=recvonly\r
m=audio 49170 RTP/AVP 0\r
m=video 51372 RTP/AVP 99\r
a=rtpmap:99 h263-1998/90000\r\n";
let origin = SdpOrigin {
    username: "jdoe".into(),
    session_id: 2890844526,
    session_version: 2890842807,
    network_type: SdpNetworkType::Internet,
    address_type: SdpAddressType::Ipv4,
    address: "10.47.16.5".into()
};
let remains = vec![];
let mut sdp_offer = SdpOffer::new(origin, "SDP Seminar");
let connection = SdpConnection {
    network_type: SdpNetworkType::Internet,
    address_type: SdpAddressType::Ipv4,
    address: "224.2.17.12/127".into()
};
let optional = vec![
     SdpSessionAttributes::Information("A Seminar on the session description protocol".into()),
     SdpSessionAttributes::Uri("http://www.example.com/seminars/sdp.pdf".into()),
     SdpSessionAttributes::Email("j.doe@example.com (Jane Doe)".into()),
     SdpSessionAttributes::Connection(connection),
     SdpSessionAttributes::Timing(SdpTiming::new(2873397496, 2873404696))
];
let attributes = vec![
    SdpAttribute { ty: SdpAttributeType::RecvOnly , value: None }
];
let medias = vec![
    SdpMedia::new(SdpMediaType::Audio, 49170, SdpProtocol::RtpAvp)
        .add_format(SdpMediaFormat::new(Codec::Pcmu)),
    SdpMedia::new(SdpMediaType::Video, 51372, SdpProtocol::RtpAvp)
        .add_format(SdpMediaFormat::new(Codec::Unknown(99)/*H263v2*/)
            .add_attribute(SdpAttribute {
                ty: SdpAttributeType::Rtpmap,
                value: Some("h263-1998/90000".into())
            })
         )
];
for attr in optional {
    sdp_offer = sdp_offer.add_optional_attributes(attr);
}
for attr in attributes {
    sdp_offer = sdp_offer.add_attribute(attr);
}
for media in medias {
    sdp_offer = sdp_offer.add_media(media);
}
println!("{}", &sdp_offer);
assert_eq!(Ok((remains.as_ref(), sdp_offer)), parse_sdp_offer(data.as_ref()));
}


#[test]
fn random_work_example() {
    let data = "v=0\r
o=bytebuddha 1303 2598 IN IP4 10.1.10.120\r
s=Talk\r
c=IN IP4 10.1.10.120\r
t=0 0\r
m=audio 7078 RTP/AVP 124 111 110 0 8 101\r
a=rtpmap:124 opus/48000\r
a=fmtp:124 useinbandfec=1; usedtx=1\r
a=rtpmap:111 speex/16000\r
a=fmtp:111 vbr=on\r
a=rtpmap:110 speex/8000\r
a=fmtp:110 vbr=on\r
a=rtpmap:101 telephone-event/8000\r
a=fmtp:101 0-11\r
m=video 9078 RTP/AVP 103 99\r
a=rtpmap:103 VP8/90000\r
a=rtpmap:99 MP4V-ES/90000\r
a=fmtp:99 profile-level-id=3\r
";
    let origin = SdpOrigin {
        username: "bytebuddha".into(),
        session_id: 1303,
        session_version: 2598,
        network_type: SdpNetworkType::Internet,
        address_type: SdpAddressType::Ipv4,
        address: "10.1.10.120".into()
    };

    let offer = SdpOffer {
        version: SdpVersion,
        origin,
        name: SdpSessionName::new("Talk"),
        optional: vec![
            SdpSessionAttributes::Connection(SdpConnection {
                network_type: SdpNetworkType::Internet,
                address_type: SdpAddressType::Ipv4,
                address: "10.1.10.120".into()
            }),
            SdpSessionAttributes::Timing(SdpTiming::new(0, 0))
        ],
        attributes: vec![],
        media: vec![
            SdpMedia {
                media: SdpMediaType::Audio,
                port: 7078,
                port_count: None,
                protocol: SdpProtocol::RtpAvp,
                attributes: vec![],
                formats: vec![
                    SdpMediaFormat {
                        codec: Codec::Unknown(124),
                        connection: None,
                        attributes: vec![
                            SdpAttribute {
                                ty: SdpAttributeType::Rtpmap,
                                value: Some("opus/48000".into()),
                            },
                            SdpAttribute {
                                ty: SdpAttributeType::Fmtp,
                                value: Some("useinbandfec=1; usedtx=1".into())
                            }
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(111),
                        connection: None,
                        attributes: vec![
                            SdpAttribute {
                                ty: SdpAttributeType::Rtpmap,
                                value: Some("speex/16000".into()),
                            },
                            SdpAttribute {
                                ty: SdpAttributeType::Fmtp,
                                value: Some("vbr=on".into()),
                            }
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(110),
                        connection: None,
                        attributes: vec![
                            SdpAttribute {
                                ty: SdpAttributeType::Rtpmap,
                                value: Some("speex/8000".into()),
                            },
                            SdpAttribute {
                                ty: SdpAttributeType::Fmtp,
                                value: Some("vbr=on".into()),
                            },
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Pcmu,
                        connection: None,
                        attributes: vec![],
                    },
                    SdpMediaFormat {
                        codec: Codec::Pcma,
                        connection: None,
                        attributes: vec![],
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(101),
                        connection: None,
                        attributes: vec![
                            SdpAttribute {
                                ty: SdpAttributeType::Rtpmap,
                                value: Some("telephone-event/8000".into()),
                            },
                            SdpAttribute {
                                ty: SdpAttributeType::Fmtp,
                                value: Some("0-11".into()),
                            },
                        ]
                    }
                ]
            },
            SdpMedia {
                media: SdpMediaType::Video,
                port: 9078,
                port_count: None,
                protocol: SdpProtocol::RtpAvp,
                attributes: vec![],
                formats: vec![
                    SdpMediaFormat {
                        codec: Codec::Unknown(103),
                        connection: None,
                        attributes: vec![
                            SdpAttribute {
                                ty: SdpAttributeType::Rtpmap,
                                value: Some("VP8/90000".into())
                            }
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(99),
                        connection: None,
                        attributes: vec![
                            SdpAttribute {
                                ty: SdpAttributeType::Rtpmap,
                                value: Some("MP4V-ES/90000".into()),
                            },
                            SdpAttribute {
                                ty: SdpAttributeType::Fmtp,
                                value: Some("profile-level-id=3".into()),
                            }
                        ]
                    }
                ]
            }
        ]
    };
    let remains = vec![];
    assert_eq!(Ok((remains.as_ref(), offer)), parse_sdp_offer(data.as_ref()));
}
