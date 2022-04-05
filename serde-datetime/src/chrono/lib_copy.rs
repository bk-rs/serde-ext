#![allow(clippy::all)]

use core::fmt;

use chrono::LocalResult;
use serde::de;

// Copy from https://github.com/chronotope/chrono/blob/v0.4.19/src/lib.rs#L603-L637
enum SerdeError<V: fmt::Display, D: fmt::Display> {
    NonExistent { timestamp: V },
    Ambiguous { timestamp: V, min: D, max: D },
}

fn ne_timestamp<T: fmt::Display>(ts: T) -> SerdeError<T, u8> {
    SerdeError::NonExistent::<T, u8> { timestamp: ts }
}

impl<V: fmt::Display, D: fmt::Display> fmt::Debug for SerdeError<V, D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChronoSerdeError({})", self)
    }
}

impl<V: fmt::Display, D: fmt::Display> fmt::Display for SerdeError<V, D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SerdeError::NonExistent { ref timestamp } => {
                write!(f, "value is not a legal timestamp: {}", timestamp)
            }
            &SerdeError::Ambiguous {
                ref timestamp,
                ref min,
                ref max,
            } => write!(
                f,
                "value is an ambiguous timestamp: {}, could be either of {}, {}",
                timestamp, min, max
            ),
        }
    }
}

// Copy from https://github.com/chronotope/chrono/blob/v0.4.19/src/datetime.rs#L1103-L1117
pub(super) fn serde_from<T, E, V>(me: LocalResult<T>, ts: &V) -> Result<T, E>
where
    E: de::Error,
    V: fmt::Display,
    T: fmt::Display,
{
    match me {
        LocalResult::None => Err(E::custom(ne_timestamp(ts))),
        LocalResult::Ambiguous(min, max) => Err(E::custom(SerdeError::Ambiguous {
            timestamp: ts,
            min: min,
            max: max,
        })),
        LocalResult::Single(val) => Ok(val),
    }
}
