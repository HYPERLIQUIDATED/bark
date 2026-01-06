use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Volume out of range: got ({current}), max ({max})")]
    VolumeOutOfRange { current: u8, max: u8 },
    #[error("Missing device key")]
    MissingDeviceKey,
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
