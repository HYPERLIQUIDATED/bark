use std::collections::HashSet;

use serde::{Serialize, ser::SerializeMap};

use crate::error::{Error, Result};

#[derive(Debug, Default)]
pub struct Flag(pub bool);

impl Flag {
    fn is_false(&self) -> bool {
        !self.0
    }
}

impl From<bool> for Flag {
    fn from(value: bool) -> Self {
        Flag(value)
    }
}

impl Serialize for Flag {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.0 {
            serializer.serialize_u8(1)
        } else {
            serializer.serialize_none()
        }
    }
}

#[derive(Debug, Default)]
pub enum BodyKind {
    #[default]
    Plaintext,
    Markdown,
}

#[derive(Debug)]
pub struct Body {
    pub kind: BodyKind,
    pub content: String,
}

impl Serialize for Body {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;

        match self.kind {
            BodyKind::Markdown => map.serialize_entry("markdown", &self.content)?,
            BodyKind::Plaintext => map.serialize_entry("body", &self.content)?,
        }

        map.end()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Level {
    Critical,
    Active,
    TimeSensitive,
    Passive,
}

#[derive(Debug, Serialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub(crate) body: Option<Body>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub(crate) device_keys: HashSet<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) level: Option<Level>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) volume: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) badge: Option<u64>,
    #[serde(skip_serializing_if = "Flag::is_false")]
    pub(crate) call: Flag,
    #[serde(skip_serializing_if = "Flag::is_false")]
    pub(crate) auto_copy: Flag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) copy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ciphertext: Option<String>,
    #[serde(skip_serializing_if = "Flag::is_false")]
    pub(crate) is_archive: Flag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<String>,
    #[serde(skip_serializing_if = "Flag::is_false")]
    pub(crate) delete: Flag,
}

impl Message {
    #[must_use]
    pub fn builder() -> MessageBuilder {
        MessageBuilder::default()
    }
}

#[derive(Default)]
pub struct MessageBuilder {
    title: Option<String>,
    subtitle: Option<String>,
    body: Option<String>,
    body_kind: BodyKind,
    device_keys: HashSet<String>,
    level: Option<Level>,
    volume: Option<u8>,
    badge: Option<u64>,
    call: bool,
    auto_copy: bool,
    copy: Option<String>,
    sound: Option<String>,
    icon: Option<String>,
    image: Option<String>,
    group: Option<String>,
    ciphertext: Option<String>,
    is_archive: bool,
    url: Option<String>,
    action: Option<String>,
    id: Option<String>,
    delete: bool,
}

impl MessageBuilder {
    const MAX_VOLUME: u8 = 10;

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    #[must_use]
    pub fn body_kind(mut self, body_kind: BodyKind) -> Self {
        self.body_kind = body_kind;
        self
    }

    #[must_use]
    pub fn device_key(mut self, device_key: impl Into<String>) -> Self {
        self.device_keys.insert(device_key.into());
        self
    }

    #[must_use]
    pub fn device_keys<I, S>(mut self, device_keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.device_keys
            .extend(device_keys.into_iter().map(Into::into));
        self
    }

    #[must_use]
    pub fn level(mut self, level: Level) -> Self {
        self.level = Some(level);
        self
    }

    #[must_use]
    pub fn volume(mut self, volume: u8) -> Self {
        self.volume = Some(volume);
        self
    }

    #[must_use]
    pub fn badge(mut self, badge: u64) -> Self {
        self.badge = Some(badge);
        self
    }

    #[must_use]
    pub fn call(mut self, call: bool) -> Self {
        self.call = call;
        self
    }

    #[must_use]
    pub fn auto_copy(mut self, auto_copy: bool) -> Self {
        self.auto_copy = auto_copy;
        self
    }

    #[must_use]
    pub fn copy(mut self, copy: impl Into<String>) -> Self {
        self.copy = Some(copy.into());
        self
    }

    #[must_use]
    pub fn sound(mut self, sound: impl Into<String>) -> Self {
        self.sound = Some(sound.into());
        self
    }

    #[must_use]
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    #[must_use]
    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }

    #[must_use]
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    #[must_use]
    pub fn ciphertext(mut self, ciphertext: impl Into<String>) -> Self {
        self.ciphertext = Some(ciphertext.into());
        self
    }

    #[must_use]
    pub fn is_archive(mut self, is_archive: bool) -> Self {
        self.is_archive = is_archive;
        self
    }

    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    #[must_use]
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use]
    pub fn delete(mut self, delete: bool) -> Self {
        self.delete = delete;
        self
    }

    pub fn build(self) -> Result<Message> {
        if let Some(volume) = self.volume.filter(|&volume| volume > Self::MAX_VOLUME) {
            return Err(Error::VolumeOutOfRange {
                current: volume,
                max: Self::MAX_VOLUME,
            });
        }

        Ok(Message {
            title: self.title,
            subtitle: self.subtitle,
            body: self.body.map(|content| Body {
                kind: self.body_kind,
                content,
            }),
            device_keys: self.device_keys,
            level: self.level,
            volume: self.volume,
            badge: self.badge,
            call: self.call.into(),
            auto_copy: self.auto_copy.into(),
            copy: self.copy,
            sound: self.sound,
            icon: self.icon,
            image: self.image,
            group: self.group,
            ciphertext: self.ciphertext,
            is_archive: self.is_archive.into(),
            url: self.url,
            action: self.action,
            id: self.id,
            delete: self.delete.into(),
        })
    }
}
