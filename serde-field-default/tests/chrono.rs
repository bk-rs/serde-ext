#![cfg(feature = "chrono")]

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[test]
fn chrono() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(default = "serde_field_default::chrono::default_date_time_utc")]
        bar: DateTime<Utc>,
    }

    //
    assert_eq!(
        serde_json::from_str::<Foo>("{}").unwrap().bar.timestamp(),
        0
    );
}
