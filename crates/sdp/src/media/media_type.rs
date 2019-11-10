pub enum MediaType {
    Audio,
    Video,
    Application
}

named!(pub parse_media_type<MediaType>, alt!(
        map!(tag!("audio"), |_| MediaType::Audio) |
        map!(tag!("video"), |_| MediaType::Video) |
        map!(tag!("application"), |_| MediaType::Application)
));
