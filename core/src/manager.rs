use sqlx::postgres::types::PgRange;
use crate::{Rsvp, RsvpManager, RsvpError, ReservationId};
use chrono::{Utc, DateTime};
use sqlx::Row;

#[async_trait::async_trait]
impl Rsvp for RsvpManager {
    async fn reserve(&self, mut rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error> {
        // rsvp::validate()?;

        let status = abi::ReservationStatus::from_i32(rsvp.status).unwrap_or(abi::ReservationStatus::Pending);

        let timespan: PgRange<DateTime<Utc>> = rsvp.get_timespan();

        // generate a insert sql for the reservation
        let sql_str = "INSERT INTO rsvp.reservation (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id";

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
