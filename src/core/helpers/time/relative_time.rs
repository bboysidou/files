use std::time::SystemTime;

use jiff::{Timestamp, tz::TimeZone};
use rust_i18n::t;

use crate::core::constants::time::{
    SECONDS_PER_DAY, SECONDS_PER_HOUR, SECONDS_PER_MINUTE, WEEKDAY_WINDOW_DAYS,
    YESTERDAY_WINDOW_DAYS,
};

pub fn relative_time(moment: SystemTime, now: SystemTime) -> String {
    let (Ok(moment), Ok(now)) = (Timestamp::try_from(moment), Timestamp::try_from(now)) else {
        return String::new();
    };
    let zone = TimeZone::system();
    let (moment, now) = (moment.to_zoned(zone.clone()), now.to_zoned(zone));
    let elapsed = now.timestamp().as_second() - moment.timestamp().as_second();
    if elapsed < SECONDS_PER_MINUTE {
        return t!("time.now").to_string();
    }
    if elapsed < SECONDS_PER_HOUR {
        return t!("time.minutes", count = elapsed / SECONDS_PER_MINUTE).to_string();
    }
    if moment.date() == now.date() {
        return format!("{:02}:{:02}", moment.hour(), moment.minute());
    }
    if elapsed < YESTERDAY_WINDOW_DAYS * SECONDS_PER_DAY {
        return t!("time.yesterday").to_string();
    }
    if elapsed < WEEKDAY_WINDOW_DAYS * SECONDS_PER_DAY {
        let key = format!("time.weekday.{}", moment.weekday().to_monday_one_offset());
        return t!(&key).to_string();
    }
    let key = format!("time.month.{}", moment.month());
    let month = t!(&key);
    if moment.year() == now.year() {
        format!("{} {month}", moment.day())
    } else {
        format!("{} {month} {}", moment.day(), moment.year())
    }
}
