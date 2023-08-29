use sqlx::postgres::types::PgRange;
use crate::{Rsvp, RsvpManager, RsvpError, ReservationId};
use chrono::{Utc, DateTime};
use sqlx::Row;

#[async_trait::async_trait]
impl Rsvp for RsvpManager {
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        // rsvp::validate()?;
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(abi::Error::InvalidTime);
        }

        let status = abi::ReservationStatus::from_i32(rsvp.status).unwrap_or(abi::ReservationStatus::Pending);

        let timespan: PgRange<DateTime<Utc>> = rsvp.get_timespan();

        // generate a insert sql for the reservation
        let sql_str = "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id";

        let id = sqlx::query(sql_str)
            .bind(rsvp.user_id.clone())
            .bind(rsvp.resource_id.clone())
            .bind(rsvp.status.to_string())
            .bind(rsvp.note.clone())
            .bind(timespan)
            .fetch_one(&self.pool)
            .await?.get(0);

        rsvp.id = id;

        Ok(rsvp)
    }

    async fn change_status(&self, rsvp_id: ReservationId) -> Result<abi::Reservation, RsvpError> {
        todo!()
    }

    async fn update_note(&self, rsvp_id: ReservationId, note: String) -> Result<abi::Reservation, RsvpError> {
        todo!()
    }

    async fn delete(&self, rsvp_id: ReservationId) -> Result<(), RsvpError> {
        todo!()
    }

    async fn get(&self, rsvp_id: ReservationId) -> Result<abi::Reservation, RsvpError> {
        todo!()
    }

    async fn query(&self, query: abi::ReservationQuery) -> Result<Vec<abi::Reservation>, RsvpError> {
        todo!()
    }
}

impl RsvpManager {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[cfg(test)]
mod tests {
    use chrono::FixedOffset;
    use abi::convert_to_timestamp;
    use super::*;

    #[sqlx_database_tester::test(
    pool(variable = "migrated_pool", migration = "../migrations"),
    )]
    async fn reserve_should_work_for_valid_window() {
        let manager = RsvpManager::new(migrated_pool.clone());
        let start: DateTime<FixedOffset> = "2023-12-25T15:00:00+0800".parse().unwrap();
        let end: DateTime<FixedOffset> = "2024-01-01T00:00:00+0800".parse().unwrap();
        let rsvpRecord = abi::Reservation {
            id: "".to_string(),
            user_id: "test".to_string(),
            resource_id: "test".to_string(),
            start: Some(convert_to_timestamp(start.with_timezone(&Utc))),
            end: Some(convert_to_timestamp(end.with_timezone(&Utc))),
            note: "TODO".to_string(),
            status: abi::ReservationStatus::Pending as i32,
        };
        let rsvp = manager.reserve(rsvpRecord).await.unwrap();
        assert(rsvp.id != "");
    }

    #[sqlx_database_tester::test(
    pool(variable = "migrated_pool", migration = "../migrations"),
    )]
    async fn reserve_should_reject_if_id_is_not_empty() {
        todo!()
    }
}
