use std::time::SystemTime;
use chrono::{DateTime, Utc};

pub(crate) fn get_now_timestamp() -> i64 {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.timestamp();
}