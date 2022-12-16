pub use chrono;

use chrono::{DateTime, NaiveDateTime, Utc};

pub fn default_date_time_utc() -> DateTime<Utc> {
    DateTime::from_utc(NaiveDateTime::from_timestamp_opt(0, 0).expect(""), Utc)
}
