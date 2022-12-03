use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    #[error("db error")]
    DbError(#[from] sqlx::Error),
    #[error("time is invalid")]
    InvalidTime,
    #[error("unknown error")]
    UnKnown,
}
