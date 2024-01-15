use std::ops::{Add, Sub};
use std::time::{Duration, SystemTime};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

const ONE_DAY_IN_SECONDS: u64 = 24 * 60 * 60;

pub(crate) fn get_now_iso_timestamp() -> String {
    now().to_rfc3339()
}

pub(crate) fn get_today_date() -> String {
    return now().format("%Y-%m-%d").to_string();
}

pub(crate) fn get_yesterday_date() -> String {
    // This function is a bit ugly... Maybe if I implement a more complex date filtering I'll refactor this
    let now = SystemTime::now().sub(
        Duration::from_secs(system_timezone_offset().add(ONE_DAY_IN_SECONDS))
    );
    let now: DateTime<Utc> = now.into();
    return now.format("%Y-%m-%d").to_string();
}

fn now() -> DateTime<FixedOffset> {
    let now = Utc::now();
    let now = now.with_timezone(
        Local::now().offset()
    );
    now
}

fn system_timezone_offset() -> u64 {
    return Local.timestamp_opt(0, 0).unwrap().offset().local_minus_utc().unsigned_abs() as u64;
}