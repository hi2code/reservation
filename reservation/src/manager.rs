use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};
use async_trait::async_trait;
use sqlx::{postgres::types::PgRange, types::chrono::DateTime, Row};

#[async_trait]
impl Rsvp for ReservationManager {
    /// make a https://plugins.jetbrains.com/intellij-platform-explorer/extensionstion
    async fn reserve(
        &self,
        mut rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }
        let start_time = abi::convert_to_utc_time(rsvp.start.as_ref().unwrap().clone());
        let end_time = abi::convert_to_utc_time(rsvp.end.as_ref().unwrap().clone());
        if start_time >= end_time {
            return Err(ReservationError::InvalidTime);
        }
        let timespan: PgRange<DateTime<_>> = (start_time..end_time).into();

        // use #[derive(sqlx::Type)] replace
        // let status = abi::ReservationStatus::from_i32(rsvp.status)
        //     .unwrap_or(abi::ReservationStatus::Pending);

        let id = sqlx::query(
            "INSERT INTO reservation (user_id, status, resource_id, timespan, note)
        VALUES ($1,$2,$3,$4,$5)",
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.status)
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .fetch_one(&self.pool)
        .await?
        .get(0);
        rsvp.id = id;
        Ok(rsvp)
    }

    /// change reservation status
    async fn change_status(
        &self,
        _id: ReservationId,
    ) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }
    /// update reservation note
    async fn update_note(
        &self,
        _id: ReservationId,
        _note: String,
    ) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }
    /// delete reservation
    async fn delete(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }
    /// get a reservation
    async fn get(&self, _id: ReservationId) -> Result<abi::Reservation, ReservationError> {
        todo!()
    }
    /// query reservations
    async fn query(
        &self,
        _query: abi::QueryReservation,
    ) -> Result<Vec<abi::Reservation>, ReservationError> {
        todo!()
    }
}
