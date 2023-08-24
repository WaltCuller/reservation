use crate::Reservation;
use super::{get_timespan};
use sqlx::{
    postgres::types::PgRange,
    types::chrono::{DateTime, Utc},
};


impl Reservation {
    pub fn get_timespan(&self) -> PgRange<DateTime<Utc>> {
        get_timespan(self.start.as_ref(), self.end.as_ref())
    }
}
