pub use chrono;

use chrono::{DateTime, Utc};

pub fn default_date_time_utc() -> DateTime<Utc> {
    DateTime::from_timestamp(0, 0).expect("Never")
}
