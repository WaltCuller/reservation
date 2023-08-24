use thiserror::Error;


#[derive(Error, Debug)]
pub enum RsvpError {
    #[error("Invalid time range for the reservation")]
    InvalidTimeRange,

    #[error("Unknown error")]
    Unknown,
}
