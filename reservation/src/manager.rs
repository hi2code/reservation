use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};
use async_trait::async_trait;
use sqlx::{types::Uuid, PgPool, Row};

#[async_trait]
impl Rsvp for ReservationManager {
    /// make a https://plugins.jetbrains.com/intellij-platform-explorer/extensionstion
    async fn reserve(
        &self,
        mut rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, ReservationError> {
        rsvp.validate()?;

        let status = abi::ReservationStatus::from_i32(rsvp.status)
            .unwrap_or(abi::ReservationStatus::Pending);

        let id: Uuid = sqlx::query(
            "INSERT INTO rsvp.reservation (user_id, status, resource_id, timespan, note)
        VALUES ($1,$2::rsvp.reservation_status,$3,$4,$5) RETURNING id",
        )
        .bind(rsvp.user_id.clone())
        .bind(status.to_string())
        .bind(rsvp.resource_id.clone())
        .bind(rsvp.get_timespan()?)
        .bind(rsvp.note.clone())
        // if use execute return the number of affected rows,we use `RETURNING id` return id of insert row
        .fetch_one(&self.pool)
        .await?
        .get(0);
        rsvp.id = id.to_string();
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

impl ReservationManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx_database_tester::test(pool(variable = "migrated_pool", migrations = "../migrations"))]
    async fn reserve_should_work_for_valid_windows() {
        let manager = ReservationManager::new(migrated_pool);
        // String that come from user client is with timezone, our system use utc;
        let rsvp = abi::Reservation::new_pending(
            "",
            "user_id",
            "r_id",
            "2022-01-01T15:00:00-0700".parse().unwrap(),
            "2022-01-04T10:00:00-0700".parse().unwrap(),
            "note",
        );
        let id = manager.reserve(rsvp).await.unwrap().id;
        assert!(!id.is_empty());
    }

    #[sqlx_database_tester::test(pool(variable = "pool", migrations = "../migrations"))]
    async fn reserve_conflict_should_reject() {
        let manager = ReservationManager::new(pool);
        let rsvp1 = abi::Reservation::new_pending(
            "",
            "alice_id",
            "room1",
            "2022-01-01T10:00:00-0700".parse().unwrap(),
            "2022-01-10T10:00:00-0700".parse().unwrap(),
            "note",
        );
        let rsvp2 = abi::Reservation::new_pending(
            "",
            "bob_id",
            "room1",
            "2022-01-01T00:00:00-0700".parse().unwrap(),
            "2022-01-05T00:00:00-0700".parse().unwrap(),
            "note",
        );
        manager.reserve(rsvp1).await.unwrap();
        let err = manager.reserve(rsvp2).await.unwrap_err();
        assert_eq!(err,ReservationError::ReservationConflict("Key (resource_id, timespan)=(room1, [\"2022-01-01 07:00:00+00\",\"2022-01-05 07:00:00+00\")) conflicts with existing key (resource_id, timespan)=(room1, [\"2022-01-01 17:00:00+00\",\"2022-01-10 17:00:00+00\")).".to_string()))
    }
}
