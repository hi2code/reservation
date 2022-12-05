use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    #[error("db error")]
    DbError(#[from] sqlx::Error),

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
