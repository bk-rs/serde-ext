use core::fmt;

use chrono::{DateTime, Utc};
use serde::{de, ser};

pub(super) struct OptionFloatMicroSecondsTimestampVisitor;
use super::ts_float_microseconds::FloatMicroSecondsTimestampVisitor;

pub fn serialize<S>(opt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    match *opt {
        Some(ref dt) => {
            let f64 =
                dt.timestamp() as f64 + f64::from(dt.timestamp_subsec_micros()) / 1_000_000_f64;
            serializer.serialize_some(&f64)
        }
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(
        d.deserialize_option(OptionFloatMicroSecondsTimestampVisitor)
            .map(|opt| opt.map(|dt| dt.with_timezone(&Utc)))?,
    )
}

impl<'de> de::Visitor<'de> for OptionFloatMicroSecondsTimestampVisitor {
    type Value = Option<DateTime<Utc>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a unix timestamp in float microseconds or none")
    }

    /// Deserialize a timestamp in float microseconds since the epoch
    fn visit_some<D>(self, d: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_i64(FloatMicroSecondsTimestampVisitor)
            .map(Some)
    }

    /// Deserialize a timestamp in float microseconds since the epoch
    fn visit_none<E>(self) -> Result<Option<DateTime<Utc>>, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    /// Deserialize a timestamp in float microseconds since the epoch
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

    use crate::chrono::ts_float_microseconds_option;

    #[test]
    fn test_ts_float_microseconds_option() -> Result<(), Box<dyn Error>> {
        #[derive(Deserialize, Serialize, Debug)]
        struct S {
            #[serde(default, with = "ts_float_microseconds_option")]
            time: Option<DateTime<Utc>>,
        }

        //
        let s: S = serde_json::from_str(r#"{ "time": null }"#)?;
        assert_eq!(s.time, None);

        let s: S = serde_json::from_str(r#"{ "time": 1609459200.999999 }"#)?;
        assert_eq!(
            s.time,
            Some(Utc.ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 999999))
        );

        let s: S = serde_json::from_str(r#"{ "time": 1609459200 }"#)?;
        assert_eq!(s.time, Some(Utc.ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 0)));

        let s: S = serde_json::from_str(r#"{ "time": 1609459200.000001 }"#)?;
        assert_eq!(s.time, Some(Utc.ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 1)));

        //
        let s = S { time: None };
        assert_eq!(serde_json::to_value(&s)?, json!({ "time": null }));

        let s = S {
            time: Some(Utc.ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 999999)),
        };
        assert_eq!(
            serde_json::to_value(&s)?,
            json!({ "time": 1609459200.999999 })
        );

        let s = S {
            time: Some(Utc.ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 0)),
        };
        assert_eq!(
            serde_json::to_value(&s)?,
            json!({ "time": 1609459200.000000 })
        );

        let s = S {
            time: Some(Utc.ymd(2021, 1, 1).and_hms_micro(0, 0, 0, 1)),
        };
        assert_eq!(
            serde_json::to_value(&s)?,
            json!({ "time": 1609459200.000001 })
        );

        Ok(())
    }
}
