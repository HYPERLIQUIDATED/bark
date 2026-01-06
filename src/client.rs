use serde::Deserialize;

use crate::{
    error::{Error, Result},
    message::Message,
};

#[derive(Debug, Deserialize)]
pub struct BarkResponse {
    pub code: i64,
    pub message: String,
    pub timestamp: i64,
}

pub struct Client {
    reqwest: reqwest::Client,
    base_url: String,
    default_device_keys: Vec<String>,
}

impl Client {
    pub fn new(base_url: impl Into<String>) -> Self {
        Client {
            reqwest: reqwest::Client::default(),
            base_url: base_url.into(),
            default_device_keys: vec![],
        }
    }

    #[must_use]
    pub fn with_device_key(mut self, device_key: impl Into<String>) -> Self {
        self.default_device_keys.push(device_key.into());
        self.default_device_keys.dedup();
        self
    }

    #[must_use]
    pub fn with_device_keys<I, S>(mut self, device_keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.default_device_keys
            .extend(device_keys.into_iter().map(Into::into));
        self.default_device_keys.dedup();
        self
    }

    pub async fn send(&self, message: &mut Message) -> Result<BarkResponse> {
        if message.device_keys.is_empty() {
            message
                .device_keys
                .extend(self.default_device_keys.iter().cloned());
        }

        if message.device_keys.is_empty() {
            return Err(Error::MissingDeviceKey);
        }

        let response = self
            .reqwest
            .post(format!("{}/push", self.base_url))
            .json(message)
            .send()
            .await?
            .error_for_status()?
            .json::<BarkResponse>()
            .await?;

        Ok(response)
    }
}
