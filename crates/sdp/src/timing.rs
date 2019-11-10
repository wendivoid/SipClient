use nom::character::is_digit;
use nirah_parse::parse_u64;

use std::time::Instant;
use std::time::Duration;

pub struct TimeZone {
    pub adjustment_type: Instant,
    pub offset: Duration
}

pub struct TimeDescription {
    pub time: Timing,
    pub repeats: Vec<Repeat>
}

pub struct Timing {
    pub start_time: u64,
    pub end_time: u64
}

named!(pub parse_timing<Timing>, do_parse!(
    start_time: map_res!(take_while!(is_digit), parse_u64) >>
    char!(' ') >>
    end_time: map_res!(take_while!(is_digit), parse_u64) >>
    (Timing { start_time: start_time.1, end_time: end_time.1 })
));

pub struct Repeat {
    pub interval: Duration,
    pub duration: Duration,
    pub offsets: Vec<Duration>
}
