use std::time::SystemTime;

use jiff::Timestamp;
use jiff::tz::TimeZone;

use crate::core::constants::files::MISSING_VALUE;
use crate::core::constants::time::ABSOLUTE_TIME_FORMAT;

pub fn absolute_time(moment: Option<SystemTime>) -> String {
    moment
        .and_then(|moment| Timestamp::try_from(moment).ok())
        .map(|stamp| {
            stamp
                .to_zoned(TimeZone::system())
                .strftime(ABSOLUTE_TIME_FORMAT)
                .to_string()
        })
        .unwrap_or_else(|| MISSING_VALUE.to_owned())
}
