
mod conflict;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid time range for the reservation")]
    InvalidTime,
    #[error("Unknown error")]
    Unknown,
}


impl From<sqlx::Error> for Error {
    fn from(_: sqlx::Error) -> Self {
        Error::Unknown
    }

}
