mod error;
mod manager;


use sqlx::PgPool;
pub use error::RsvpError;

pub type ReservationId = String;
pub type UserId = String;
pub type ResourceId = String;

#[async_trait::async_trait]
pub trait Rsvp {
    async fn reserve(&self, rsvp: abi::Reservation) -> Result<abi::Reservation, abi::Error>;

    async fn change_status(&self, rsvp_id: ReservationId) -> Result<abi::Reservation, RsvpError>;
    /// update note
    async fn update_note(&self, rsvp_id: ReservationId, note: String) -> Result<abi::Reservation, RsvpError>;
    /// delete reservation
    async fn delete(&self, rsvp_id: ReservationId) -> Result<(), RsvpError>;
    /// get reservation by id
    async fn get(&self, rsvp_id: ReservationId) -> Result<abi::Reservation, RsvpError>;
    /// query reservations
    async fn query(&self, query: abi::ReservationQuery) -> Result<Vec<abi::Reservation>, RsvpError>;
}


#[derive(Debug)]
pub struct RsvpManager {
  pool: PgPool,
}
