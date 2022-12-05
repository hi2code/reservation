use chrono::{DateTime, NaiveDateTime, Utc};
use prost_types::Timestamp;

use crate::ReservationError;

pub fn convert_to_utc_time(ts: Timestamp) -> Result<DateTime<Utc>, ReservationError> {
    let datetime = NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as _)
        .ok_or(ReservationError::InvalidTimestamp)?;
    Ok(DateTime::<Utc>::from_utc(datetime, Utc))
}

pub fn convert_utc_to_timestamp(t: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: t.timestamp(),
        nanos: t.timestamp_subsec_nanos() as _,
    }
}
