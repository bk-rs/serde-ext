#![cfg(feature = "chrono-tz")]

use chrono_tz::Tz;
use serde::Deserialize;

#[test]
fn chrono_tz() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(
            default = "serde_field_default::chrono_tz::default_tz",
            deserialize_with = "deserialize"
        )]
        bar: Tz,
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<Tz, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = Box::<str>::deserialize(deserializer)?;
        s.parse::<Tz>().map_err(serde::de::Error::custom)
    }

    assert_eq!(serde_json::from_str::<Foo>("{}").unwrap().bar, Tz::GMT);
}
