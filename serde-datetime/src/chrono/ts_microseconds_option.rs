use core::fmt;

use chrono::{DateTime, Utc};
use serde::{de, ser};

use super::ext::DTTimestampMicrosExt as _;

pub(super) struct OptionMicroSecondsTimestampVisitor;
use super::ts_microseconds::MicroSecondsTimestampVisitor;

pub fn serialize<S>(opt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    match *opt {
        Some(ref dt) => serializer.serialize_some(&dt.timestamp_micros()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(d.deserialize_option(OptionMicroSecondsTimestampVisitor)
        .map(|opt| opt.map(|dt| dt.with_timezone(&Utc)))?)
}

impl<'de> de::Visitor<'de> for OptionMicroSecondsTimestampVisitor {
    type Value = Option<DateTime<Utc>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a unix timestamp in microseconds or none")
    }

    /// Deserialize a timestamp in microseconds since the epoch
    fn visit_some<D>(self, d: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_i64(MicroSecondsTimestampVisitor).map(Some)
    }

    /// Deserialize a timestamp in microseconds since the epoch
    fn visit_none<E>(self) -> Result<Option<DateTime<Utc>>, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    /// Deserialize a timestamp in microseconds since the epoch
    fn visit_unit<E>(self) -> Result<Option<DateTime<Utc>>, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use chrono::{DateTime, TimeZone as _, Utc};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::chrono::ts_microseconds_option;

    #[test]
    fn test_ts_microseconds_option() -> Result<(), Box<dyn Error>> {
        #[derive(Deserialize, Serialize, Debug)]
        struct S {
            #[serde(default, with = "ts_microseconds_option")]
            time: Option<DateTime<Utc>>,
        }

        //
        let s: S = serde_json::from_str(r#"{ "time": null }"#)?;
        assert_eq!(s.time, None);

        let s: S = serde_json::from_str(r#"{ "time": 1609459200999999 }"#)?;
        assert_eq!(
            s.time,
            Some(Utc.ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 999999))
        );

        //
        let s = S { time: None };
        assert_eq!(serde_json::to_value(&s)?, json!({ "time": null }));

        let s = S {
            time: Some(Utc.ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 999999)),
        };
        assert_eq!(
            serde_json::to_value(&s)?,
            json!({ "time": 1609459200999999_u64 })
        );

        Ok(())
    }
}
