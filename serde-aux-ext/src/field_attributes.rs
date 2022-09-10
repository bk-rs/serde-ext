/*
Copyright (c) 2018 Victor Polevoy. All Rights Reserved.

Distributed under MIT license. See https://github.com/vityafx/serde-aux/blob/master/LICENSE for detail.
*/

use serde::{Deserialize, Deserializer};

//
pub fn deserialize_option_bool_from_anything<'de, D>(
    deserializer: D,
) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    use core::f64::EPSILON;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AnythingOrBoolOrNull {
        String(String),
        Int(i64),
        Float(f64),
        Boolean(bool),
        Null,
    }

    match AnythingOrBoolOrNull::deserialize(deserializer)? {
        AnythingOrBoolOrNull::Boolean(b) => Ok(Some(b)),
        AnythingOrBoolOrNull::Int(i) => match i {
            1 => Ok(Some(true)),
            0 => Ok(Some(false)),
            _ => Err(serde::de::Error::custom("The number is neither 1 nor 0")),
        },
        AnythingOrBoolOrNull::Float(f) => {
            if (f - 1.0f64).abs() < EPSILON {
                Ok(Some(true))
            } else if f == 0.0f64 {
                Ok(Some(false))
            } else {
                Err(serde::de::Error::custom(
                    "The number is neither 1.0 nor 0.0",
                ))
            }
        }
        AnythingOrBoolOrNull::String(string) => {
            if let Ok(b) = string.parse::<bool>() {
                Ok(Some(b))
            } else if let Ok(i) = string.parse::<i64>() {
                match i {
                    1 => Ok(Some(true)),
                    0 => Ok(Some(false)),
                    _ => Err(serde::de::Error::custom("The number is neither 1 nor 0")),
                }
            } else if let Ok(f) = string.parse::<f64>() {
                if (f - 1.0f64).abs() < EPSILON {
                    Ok(Some(true))
                } else if f == 0.0f64 {
                    Ok(Some(false))
                } else {
                    Err(serde::de::Error::custom(
                        "The number is neither 1.0 nor 0.0",
                    ))
                }
            } else {
                Err(serde::de::Error::custom(format!(
                    "Could not parse boolean from a string: {}",
                    string
                )))
            }
        }
        AnythingOrBoolOrNull::Null => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_option_bool_from_anything() {
        #[derive(Deserialize, Debug)]
        struct Struct {
            #[serde(deserialize_with = "deserialize_option_bool_from_anything")]
            field: Option<bool>,
        }

        //
        assert_eq!(
            serde_json::from_str::<Struct>(r#"{"field": null}"#)
                .unwrap()
                .field,
            None
        );

        //
        assert_eq!(
            serde_json::from_str::<Struct>(r#"{"field": true}"#)
                .unwrap()
                .field,
            Some(true)
        );
        assert_eq!(
            serde_json::from_str::<Struct>(r#"{"field": false}"#)
                .unwrap()
                .field,
            Some(false)
        );

        //
        assert_eq!(
            serde_json::from_str::<Struct>(r#"{"field": "true"}"#)
                .unwrap()
                .field,
            Some(true)
        );
        assert_eq!(
            serde_json::from_str::<Struct>(r#"{"field": "false"}"#)
                .unwrap()
                .field,
            Some(false)
        );

        //
        assert_eq!(
            serde_json::from_str::<Struct>(r#"{"field": 1}"#)
                .unwrap()
                .field,
            Some(true)
        );
        assert_eq!(
            serde_json::from_str::<Struct>(r#"{"field": 0}"#)
                .unwrap()
                .field,
            Some(false)
        );
    }
}
