use thiserror::Error;

use crate::response::PushResponse;

#[derive(Debug, Error)]
pub enum Error {
    #[error("volume out of range: got {current}, max {max}")]
    VolumeOutOfRange { current: u8, max: u8 },

    #[error("badge out of range: got {current}, max {max}")]
    BadgeOutOfRange { current: u64, max: u64 },

    #[error("missing device key")]
    MissingDeviceKey,

    #[error("device keys must not be empty")]
    InvalidDeviceKey,

    #[error("deleting a notification requires a nonempty id")]
    MissingNotificationId,

    #[error("invalid server URL: {0}")]
    InvalidBaseUrl(String),

    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("HTTP {status}: {body}")]
    Http {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("invalid Bark response: {0}")]
    Decode(#[from] serde_json::Error),

    #[error("Bark response does not match the requested recipients: {response:?}")]
    UnexpectedResponse { response: PushResponse },

    #[error("Bark rejected the request (HTTP {status}): {response:?}")]
    Api {
        status: reqwest::StatusCode,
        response: PushResponse,
    },

    #[error("one or more devices rejected the push: {response:?}")]
    Batch { response: PushResponse },
}

pub type Result<T> = std::result::Result<T, Error>;
