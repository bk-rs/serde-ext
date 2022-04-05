use chrono::{DateTime, NaiveDateTime, TimeZone};

pub(super) trait DTTimestampMicrosExt {
    fn timestamp_micros(&self) -> i64;
}

impl<Tz: TimeZone> DTTimestampMicrosExt for DateTime<Tz> {
    fn timestamp_micros(&self) -> i64 {
        let as_ns = self.timestamp() * 1_000_000;
        as_ns + i64::from(self.timestamp_subsec_micros())
    }
}

impl DTTimestampMicrosExt for NaiveDateTime {
    fn timestamp_micros(&self) -> i64 {
        let as_ns = self.timestamp() * 1_000_000;
        as_ns + i64::from(self.timestamp_subsec_micros())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{NaiveDate, Utc};

    #[test]
    fn test_timestamp_micros() {
        let dt = NaiveDate::from_ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 999999);
        assert_eq!(dt.timestamp_micros(), 1609459200999999);

        let dt = DateTime::<Utc>::from_utc(dt, Utc);
        assert_eq!(dt.timestamp_micros(), 1609459200999999);
    }
}
