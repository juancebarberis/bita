use std::time::SystemTime;
use chrono::{DateTime, Utc};

pub(crate) fn get_now_timestamp() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.to_rfc3339();
}