use std::collections::HashSet;

use serde::Deserialize;

use crate::{
    error::{Error, Result},
    message::{BodyKind, Level, Message, MessageBuilder},
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
    default_device_keys: HashSet<String>,
}

impl Client {
    pub fn new(base_url: impl Into<String>) -> Self {
        let mut base_url = base_url.into();

        if base_url.ends_with('/') {
            base_url.pop();
        }

        Client {
            reqwest: reqwest::Client::default(),
            base_url,
            default_device_keys: HashSet::default(),
        }
    }

    #[must_use]
    pub fn with_device_key(mut self, device_key: impl Into<String>) -> Self {
        self.default_device_keys.insert(device_key.into());
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
        self
    }

    pub fn message(&self) -> ClientMessageBuilder<'_> {
        ClientMessageBuilder {
            client: self,
            builder: MessageBuilder::default(),
        }
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

pub struct ClientMessageBuilder<'a> {
    client: &'a Client,
    builder: MessageBuilder,
}

impl ClientMessageBuilder<'_> {
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.builder = self.builder.title(title);
        self
    }

    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.builder = self.builder.subtitle(subtitle);
        self
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.builder = self.builder.body(body);
        self
    }

    #[must_use]
    pub fn body_kind(mut self, body_kind: BodyKind) -> Self {
        self.builder = self.builder.body_kind(body_kind);
        self
    }

    #[must_use]
    pub fn device_key(mut self, device_key: impl Into<String>) -> Self {
        self.builder = self.builder.device_key(device_key);
        self
    }

    #[must_use]
    pub fn device_keys<I, S>(mut self, device_keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.builder = self.builder.device_keys(device_keys);
        self
    }

    #[must_use]
    pub fn level(mut self, level: Level) -> Self {
        self.builder = self.builder.level(level);
        self
    }

    #[must_use]
    pub fn volume(mut self, volume: u8) -> Self {
        self.builder = self.builder.volume(volume);
        self
    }

    #[must_use]
    pub fn badge(mut self, badge: u64) -> Self {
        self.builder = self.builder.badge(badge);
        self
    }

    #[must_use]
    pub fn call(mut self, call: bool) -> Self {
        self.builder = self.builder.call(call);
        self
    }

    #[must_use]
    pub fn auto_copy(mut self, auto_copy: bool) -> Self {
        self.builder = self.builder.auto_copy(auto_copy);
        self
    }

    #[must_use]
    pub fn copy(mut self, copy: impl Into<String>) -> Self {
        self.builder = self.builder.copy(copy);
        self
    }

    #[must_use]
    pub fn sound(mut self, sound: impl Into<String>) -> Self {
        self.builder = self.builder.sound(sound);
        self
    }

    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.builder = self.builder.icon(icon);
        self
    }

    #[must_use]
    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.builder = self.builder.image(image);
        self
    }

    #[must_use]
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.builder = self.builder.group(group);
        self
    }

    #[must_use]
    pub fn ciphertext(mut self, ciphertext: impl Into<String>) -> Self {
        self.builder = self.builder.ciphertext(ciphertext);
        self
    }

    #[must_use]
    pub fn is_archive(mut self, is_archive: bool) -> Self {
        self.builder = self.builder.is_archive(is_archive);
        self
    }

    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.builder = self.builder.url(url);
        self
    }

    #[must_use]
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.builder = self.builder.action(action);
        self
    }

    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.builder = self.builder.id(id);
        self
    }

    #[must_use]
    pub fn delete(mut self, delete: bool) -> Self {
        self.builder = self.builder.delete(delete);
        self
    }

    pub async fn send(self) -> Result<BarkResponse> {
        let mut message = self.builder.build()?;
        self.client.send(&mut message).await
    }
}
