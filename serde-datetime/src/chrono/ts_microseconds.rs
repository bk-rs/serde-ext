use core::fmt;

use chrono::{DateTime, TimeZone as _, Utc};
use serde::{de, ser};

use super::ext::DTTimestampMicrosExt as _;
use super::lib_copy::serde_from;

pub(crate) struct MicroSecondsTimestampVisitor;

pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    serializer.serialize_i64(dt.timestamp_micros())
}

pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(d.deserialize_i64(MicroSecondsTimestampVisitor)
        .map(|dt| dt.with_timezone(&Utc))?)
}

impl<'de> de::Visitor<'de> for MicroSecondsTimestampVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a unix timestamp in microseconds")
    }

    /// Deserialize a timestamp in microseconds since the epoch
    fn visit_i64<E>(self, value: i64) -> Result<DateTime<Utc>, E>
    where
        E: de::Error,
    {
        serde_from(
            Utc.timestamp_opt(value / 1_000_000, ((value % 1_000_000) * 1_000) as u32),
            &value,
        )
    }

    /// Deserialize a timestamp in microseconds since the epoch
    fn visit_u64<E>(self, value: u64) -> Result<DateTime<Utc>, E>
    where
        E: de::Error,
    {
        serde_from(
            Utc.timestamp_opt(
                (value / 1_000_000) as i64,
                ((value % 1_000_000) * 1_000) as u32,
            ),
            &value,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use chrono::{DateTime, TimeZone as _, Utc};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::chrono::ts_microseconds;

    #[test]
    fn test_ts_microseconds() -> Result<(), Box<dyn Error>> {
        #[derive(Deserialize, Serialize, Debug)]
        struct S {
            #[serde(with = "ts_microseconds")]
            time: DateTime<Utc>,
        }

        //
        let s: S = serde_json::from_str(r#"{ "time": 1609459200999999 }"#)?;
        assert_eq!(s.time, Utc.ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 999999));

        //
        let s = S {
            time: Utc.ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 999999),
        };
        assert_eq!(
            serde_json::to_value(&s)?,
            json!({ "time": 1609459200999999_u64 })
        );

        Ok(())
    }
}
