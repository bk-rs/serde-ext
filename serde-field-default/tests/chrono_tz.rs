#![cfg(feature = "chrono-tz")]

use chrono_tz::Tz;
use serde::{de, Deserialize, Deserializer};

#[test]
fn chrono_tz() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(
            default = "serde_field_default::chrono_tz::default_tz",
            deserialize_with = "my_deserialize"
        )]
        bar: Tz,
    }

    fn my_deserialize<'de, D>(deserializer: D) -> Result<Tz, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Box::<str>::deserialize(deserializer)?;
        s.parse::<Tz>().map_err(de::Error::custom)
    }

    //
    assert_eq!(serde_json::from_str::<Foo>("{}").unwrap().bar, Tz::GMT);
}
