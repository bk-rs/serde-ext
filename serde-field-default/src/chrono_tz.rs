pub use chrono_tz;

use chrono_tz::Tz;

pub fn default_tz() -> Tz {
    Tz::GMT
}
