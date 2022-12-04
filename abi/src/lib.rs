mod error;
mod pb;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
pub use error::*;
pub use pb::*;
use prost_types::Timestamp;
use std::fmt::Display;

pub fn convert_to_utc_time(ts: Timestamp) -> Result<DateTime<Utc>, ReservationError> {
    let datetime = NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as _)
        .ok_or(ReservationError::InvalidTime)?;
    Ok(DateTime::<Utc>::from_utc(datetime, Utc))
}

pub fn convert_utc_to_timestamp(t: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: t.timestamp(),
        nanos: t.timestamp_subsec_nanos() as _,
    }
}

impl Reservation {
    pub fn new_pending(
        id: impl Into<String>,
        uid: impl Into<String>,
        rid: impl Into<String>,
        start: DateTime<FixedOffset>,
        end: DateTime<FixedOffset>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            user_id: uid.into(),
            status: ReservationStatus::Pending as i32,
            resource_id: rid.into(),
            start: Some(convert_utc_to_timestamp(start.with_timezone(&Utc))),
            end: Some(convert_utc_to_timestamp(end.with_timezone(&Utc))),
            note: note.into(),
        }
    }
}

impl Display for ReservationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReservationStatus::Unknown => write!(f, "unknown"),
            ReservationStatus::Pending => write!(f, "pending"),
            ReservationStatus::Confirmed => write!(f, "confirmed"),
            ReservationStatus::Blocked => write!(f, "blocked"),
        }
    }
}
