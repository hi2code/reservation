use chrono::{DateTime, FixedOffset, Utc};
use sqlx::postgres::types::PgRange;

use crate::{
    convert_to_utc_time, convert_utc_to_timestamp, Reservation, ReservationError, ReservationStatus,
};

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

    pub fn validate(&self) -> Result<(), ReservationError> {
        if self.start.is_none() || self.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }
        if self.resource_id.is_empty() {
            return Err(ReservationError::InvalidResourceId(
                self.resource_id.clone(),
            ));
        }
        if self.user_id.is_empty() {
            return Err(ReservationError::InvalidUserId(self.user_id.clone()));
        }
        let start_time = convert_to_utc_time(self.start.as_ref().unwrap().clone())?;
        let end_time = convert_to_utc_time(self.end.as_ref().unwrap().clone())?;
        if start_time >= end_time {
            return Err(ReservationError::InvalidTime);
        }
        Ok(())
    }

    pub fn get_timespan(&self) -> Result<PgRange<DateTime<Utc>>, ReservationError> {
        let start_time = convert_to_utc_time(self.start.as_ref().unwrap().clone())?;
        let end_time = convert_to_utc_time(self.end.as_ref().unwrap().clone())?;
        if start_time >= end_time {
            return Err(ReservationError::InvalidTime);
        }
        Ok((start_time..end_time).into())
    }
}
