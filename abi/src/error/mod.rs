mod conflict;
pub use conflict::{ReservationConflictInfo, ReservationWindow};
use sqlx::postgres::PgDatabaseError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ReservationError {
    #[error("db error")]
    DbError(sqlx::Error),

    #[error("reservation conflict")]
    ReservationConflict(ReservationConflictInfo),

    #[error("time is invalid")]
    InvalidTime,

    #[error("timestamp is over")]
    InvalidTimestamp,

    #[error("invalid user id : {0}")]
    InvalidUserId(String),

    #[error("invalid resource id: {0}")]
    InvalidResourceId(String),

    #[error("unknown error")]
    UnKnown,
}

impl From<sqlx::Error> for ReservationError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Database(e) => {
                let err: &PgDatabaseError = e.downcast_ref();
                match (err.code(), err.schema(), err.table()) {
                    ("23P01", Some("rsvp"), Some("reservation")) => {
                        ReservationError::ReservationConflict(
                            err.detail().unwrap().parse().unwrap(),
                        )
                    }
                    _ => ReservationError::DbError(sqlx::Error::Database(e)),
                }
            }
            _ => ReservationError::DbError(e),
        }
    }
}

impl PartialEq for ReservationError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ReservationConflict(l0), Self::ReservationConflict(r0)) => l0 == r0,
            (Self::InvalidUserId(l0), Self::InvalidUserId(r0)) => l0 == r0,
            (Self::InvalidResourceId(l0), Self::InvalidResourceId(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
