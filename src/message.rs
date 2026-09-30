use std::collections::BTreeSet;

use serde::{Serialize, Serializer, ser::SerializeMap};

use crate::error::{Error, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
enum Flag {
    #[serde(rename = "1")]
    Enabled,
    #[serde(rename = "0")]
    Disabled,
}

impl From<bool> for Flag {
    fn from(enabled: bool) -> Self {
        if enabled {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

#[derive(Clone, Debug, Serialize)]
enum Body {
    #[serde(rename = "body")]
    Text(String),
    #[serde(rename = "markdown")]
    Markdown(String),
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(into = "String")]
struct Badge(u64);

impl From<Badge> for String {
    fn from(badge: Badge) -> Self {
        badge.0.to_string()
    }
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Level {
    Critical,
    Active,
    TimeSensitive,
    Passive,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Alert,
    None,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Message {
    #[serde(flatten, serialize_with = "serialize_device_keys")]
    pub(crate) device_keys: BTreeSet<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<String>,
    #[serde(flatten)]
    body: Option<Body>,

    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<Level>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    call: Option<Flag>,

    #[serde(skip_serializing_if = "Option::is_none")]
    badge: Option<Badge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "autoCopy")]
    auto_copy: Option<Flag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Action>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ciphertext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iv: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "isArchive")]
    archive: Option<Flag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<Flag>,
}

impl Message {
    pub fn builder() -> MessageBuilder {
        MessageBuilder::default()
    }
}

#[must_use]
#[derive(Clone, Debug, Default)]
pub struct MessageBuilder {
    message: Message,
}

impl MessageBuilder {
    pub fn device_key(mut self, device_key: impl Into<String>) -> Self {
        self.message.device_keys.insert(device_key.into());
        self
    }

    pub fn device_keys(mut self, device_keys: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.message
            .device_keys
            .extend(device_keys.into_iter().map(Into::into));
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.message.title = Some(title.into());
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.message.subtitle = Some(subtitle.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.message.body = Some(Body::Text(body.into()));
        self
    }

    pub fn markdown(mut self, markdown: impl Into<String>) -> Self {
        self.message.body = Some(Body::Markdown(markdown.into()));
        self
    }

    pub fn level(mut self, level: Level) -> Self {
        self.message.level = Some(level);
        self
    }

    pub fn volume(mut self, volume: u8) -> Self {
        self.message.volume = Some(volume);
        self
    }

    pub fn sound(mut self, sound: impl Into<String>) -> Self {
        self.message.sound = Some(sound.into());
        self
    }

    pub fn call(mut self, call: bool) -> Self {
        self.message.call = Some(call.into());
        self
    }

    pub fn badge(mut self, badge: u64) -> Self {
        self.message.badge = Some(Badge(badge));
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.message.icon = Some(icon.into());
        self
    }

    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.message.image = Some(image.into());
        self
    }

    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.message.group = Some(group.into());
        self
    }

    pub fn auto_copy(mut self, auto_copy: bool) -> Self {
        self.message.auto_copy = Some(auto_copy.into());
        self
    }

    pub fn copy(mut self, copy: impl Into<String>) -> Self {
        self.message.copy = Some(copy.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.message.url = Some(url.into());
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.message.action = Some(action);
        self
    }

    pub fn ciphertext(mut self, ciphertext: impl Into<String>) -> Self {
        self.message.ciphertext = Some(ciphertext.into());
        self
    }

    pub fn iv(mut self, iv: impl Into<String>) -> Self {
        self.message.iv = Some(iv.into());
        self
    }

    pub fn archive(mut self, archive: bool) -> Self {
        self.message.archive = Some(archive.into());
        self
    }

    pub fn ttl(mut self, ttl: u64) -> Self {
        self.message.ttl = Some(ttl);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.message.id = Some(id.into());
        self
    }

    pub fn delete(mut self, delete: bool) -> Self {
        self.message.delete = Some(delete.into());
        self
    }

    /// # Errors
    /// Returns an error for volume above 10, badge above `i64::MAX`, empty device keys,
    /// or deletion without an ID containing non-whitespace characters.
    pub fn build(self) -> Result<Message> {
        if let Some(volume) = self.message.volume.filter(|&volume| volume > 10) {
            return Err(Error::VolumeOutOfRange {
                current: volume,
                max: 10,
            });
        }

        let max_badge = i64::MAX.unsigned_abs();

        if let Some(badge) = self.message.badge.filter(|badge| badge.0 > max_badge) {
            return Err(Error::BadgeOutOfRange {
                current: badge.0,
                max: max_badge,
            });
        }

        validate_device_keys(&self.message.device_keys)?;

        if self.message.delete == Some(Flag::Enabled)
            && self
                .message
                .id
                .as_ref()
                .is_none_or(|id| id.trim().is_empty())
        {
            return Err(Error::MissingNotificationId);
        }

        Ok(self.message)
    }
}

pub(crate) fn validate_device_keys(keys: &BTreeSet<String>) -> Result<()> {
    if keys.contains("") {
        return Err(Error::InvalidDeviceKey);
    }

    Ok(())
}

fn serialize_device_keys<S>(
    keys: &BTreeSet<String>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut recipients = serializer.serialize_map(Some(usize::from(!keys.is_empty())))?;

    if let Some(key) = keys.first() {
        if keys.len() == 1 {
            recipients.serialize_entry("device_key", key)?;
        } else {
            recipients.serialize_entry("device_keys", keys)?;
        }
    }

    recipients.end()
}
