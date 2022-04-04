pub use chrono;

use chrono::{DateTime, NaiveDateTime, Utc};

pub fn default_date_time_utc() -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
}
