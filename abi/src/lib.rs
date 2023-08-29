mod pb;
mod types;
mod error;
mod utils;

use prost_types::Timestamp;
pub use error::{Error};
pub use pb::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use utils::*;
use std::fmt;
use std::fmt::Formatter;

pub fn convert_to_utc_time(ts: Timestamp) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as _).unwrap(),
        Utc,
    )
}

pub fn convert_to_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as _,
    }
}

impl fmt::Display for ReservationStatus  {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReservationStatus::Pending => write!(f, "Pending"),
            ReservationStatus::Confirmed => write!(f, "Confirmed"),
            ReservationStatus::Blocked => write!(f, "Blocked"),
            ReservationStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Reservation {
    pub fn new_pending() -> Self {
        Self {
            id: 0,
            user_id: "".to_string(),
            resource_id: "".to_string(),
            start: None,
            end: None,
            note: "".to_string(),
            status: ReservationStatus::Pending as _,
        }
    }
}
