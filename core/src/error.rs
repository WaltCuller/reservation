use thiserror::Error;


#[derive(Error, Debug)]
pub enum RsvpError {

    #[error("Invalid SQLX error")]
    DBError(#[from] sqlx::Error),

    #[error("Invalid time range for the reservation")]
    InvalidTimeRange,

    #[error("Unknown error")]
    Unknown,
}
