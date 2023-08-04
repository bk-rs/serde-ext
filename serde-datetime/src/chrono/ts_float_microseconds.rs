use core::fmt;

use chrono::{DateTime, Datelike as _, NaiveDate, TimeZone as _, Timelike as _, Utc};
use serde::{de, ser};

use super::lib_copy::serde_from;

pub(crate) struct FloatMicroSecondsTimestampVisitor;

pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    let f64 = dt.timestamp() as f64 + f64::from(dt.timestamp_subsec_micros()) / 1_000_000_f64;
    serializer.serialize_f64(f64)
}

pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_f64(FloatMicroSecondsTimestampVisitor)
        .map(|dt| dt.with_timezone(&Utc))
}

impl<'de> de::Visitor<'de> for FloatMicroSecondsTimestampVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a unix timestamp in float microseconds")
    }

    /// Deserialize a timestamp in float microseconds since the epoch
    fn visit_i64<E>(self, value: i64) -> Result<DateTime<Utc>, E>
    where
        E: de::Error,
    {
        serde_from(Utc.timestamp_opt(value, 0), &value)
    }

    /// Deserialize a timestamp in float microseconds since the epoch
    fn visit_u64<E>(self, value: u64) -> Result<DateTime<Utc>, E>
    where
        E: de::Error,
    {
        serde_from(Utc.timestamp_opt(value as i64, 0), &value)
    }

    /// Deserialize a timestamp in float microseconds since the epoch
    fn visit_f64<E>(self, value: f64) -> Result<DateTime<Utc>, E>
    where
        E: de::Error,
    {
        serde_from(
            Utc.timestamp_opt(
                value as i64,
                ((value * 1_000_000_f64) as u64 % 1_000_000) as u32,
            ),
            &value,
        )
        .map(|dt| {
            DateTime::from_utc(
                NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day())
                    .expect("")
                    .and_hms_micro_opt(dt.hour(), dt.minute(), dt.second(), dt.nanosecond())
                    .expect(""),
                Utc,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, Utc};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::chrono::ts_float_microseconds;

    #[test]
    fn test_ts_float_microseconds() -> Result<(), serde_json::Error> {
        #[derive(Deserialize, Serialize, Debug)]
        struct S {
            #[serde(with = "ts_float_microseconds")]
            time: DateTime<Utc>,
        }

        //
        let s: S = serde_json::from_str(r#"{ "time": 1609459200.999999 }"#)?;
        assert_eq!(
            s.time,
            DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd_opt(2021, 1, 1)
                    .expect("")
                    .and_hms_micro_opt(0, 0, 0, 999999)
                    .expect(""),
                Utc
            )
        );

        let s: S = serde_json::from_str(r#"{ "time": 1609459200 }"#)?;
        assert_eq!(
            s.time,
            DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd_opt(2021, 1, 1)
                    .expect("")
                    .and_hms_micro_opt(0, 0, 0, 0)
                    .expect(""),
                Utc
            )
        );

        let s: S = serde_json::from_str(r#"{ "time": 1609459200.000001 }"#)?;
        assert_eq!(
            s.time,
            DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd_opt(2021, 1, 1)
                    .expect("")
                    .and_hms_micro_opt(0, 0, 0, 1)
                    .expect(""),
                Utc
            )
        );

        //
        let s = S {
            time: DateTime::from_utc(
                NaiveDate::from_ymd_opt(2021, 1, 1)
                    .expect("")
                    .and_hms_micro_opt(0, 0, 0, 999999)
                    .expect(""),
                Utc,
            ),
        };
        assert_eq!(
            serde_json::to_value(s)?,
            json!({ "time": 1609459200.999999 })
        );

        let s = S {
            time: DateTime::from_utc(
                NaiveDate::from_ymd_opt(2021, 1, 1)
                    .expect("")
                    .and_hms_micro_opt(0, 0, 0, 0)
                    .expect(""),
                Utc,
            ),
        };
        assert_eq!(
            serde_json::to_value(s)?,
            json!({ "time": 1609459200.000000 })
        );

        let s = S {
            time: DateTime::from_utc(
                NaiveDate::from_ymd_opt(2021, 1, 1)
                    .expect("")
                    .and_hms_micro_opt(0, 0, 0, 1)
                    .expect(""),
                Utc,
            ),
        };
        assert_eq!(
            serde_json::to_value(s)?,
            json!({ "time": 1609459200.000001 })
        );

        Ok(())
    }
}
