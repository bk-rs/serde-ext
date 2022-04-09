#![cfg(feature = "alloc")]

use chrono_tz::Tz;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[test]
fn to_and_from_string() {
    #[derive(Deserialize, Serialize)]
    struct Foo {
        #[serde(with = "serde_field_with::to_and_from_string")]
        tz1: Tz,
        #[serde(
            serialize_with = "serde_field_with::to_string",
            deserialize_with = "my_deserialize"
        )]
        tz2: Tz,
        #[serde(
            serialize_with = "my_serialize",
            deserialize_with = "serde_field_with::from_str"
        )]
        tz3: Tz,
        #[serde(
            default,
            with = "serde_field_with::to_and_from_string_option",
            skip_serializing_if = "Option::is_none"
        )]
        tz4: Option<Tz>,
        #[serde(with = "serde_field_with::to_and_from_string_option")]
        tz5: Option<Tz>,
    }

    fn my_deserialize<'de, D>(deserializer: D) -> Result<Tz, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Box::<str>::deserialize(deserializer)?;
        debug_assert_eq!(s, "GMT".into());
        Ok(Tz::Asia__Shanghai)
    }

    fn my_serialize<S>(tz: &Tz, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        debug_assert_eq!(tz, &Tz::GMT);
        "Europe/London".serialize(serializer)
    }

    //
    let x = serde_json::from_str::<Foo>(
        r#"{"tz1":"America/New_York","tz2":"GMT","tz3":"Europe/London","tz5":"Asia/Shanghai"}"#,
    )
    .unwrap();
    assert_eq!(x.tz1, Tz::America__New_York);
    assert_eq!(x.tz2, Tz::Asia__Shanghai);
    assert_eq!(x.tz3, Tz::Europe__London);
    assert_eq!(x.tz4, None);
    assert_eq!(x.tz5, Some(Tz::Asia__Shanghai));

    //
    let x = Foo {
        tz1: Tz::America__New_York,
        tz2: Tz::Asia__Shanghai,
        tz3: Tz::GMT,
        tz4: None,
        tz5: Some(Tz::Asia__Shanghai),
    };
    assert_eq!(
        serde_json::to_string(&x).unwrap(),
        r#"{"tz1":"America/New_York","tz2":"Asia/Shanghai","tz3":"Europe/London","tz5":"Asia/Shanghai"}"#
    );
}
