use serde::{Serialize, ser::SerializeMap};

use crate::error::{Error, Result};

#[derive(Debug, Default)]
pub enum BodyType {
    #[default]
    Plaintext,
    Markdown,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Level {
    Critical,
    Active,
    TimeSensitive,
    Passive,
}

#[derive(Debug)]
pub struct Message {
    pub(crate) title: Option<String>,
    pub(crate) subtitle: Option<String>,
    pub(crate) body: Option<String>,
    pub(crate) body_type: BodyType,
    pub(crate) device_keys: Vec<String>,
    pub(crate) level: Option<Level>,
    pub(crate) volume: Option<u8>,
    pub(crate) badge: Option<u64>,
    pub(crate) call: Option<bool>,
    pub(crate) auto_copy: Option<bool>,
    pub(crate) copy: Option<String>,
    pub(crate) sound: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) image: Option<String>,
    pub(crate) group: Option<String>,
    pub(crate) ciphertext: Option<String>,
    pub(crate) is_archive: Option<bool>,
    pub(crate) url: Option<String>,
    pub(crate) action: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) delete: Option<bool>,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        macro_rules! serialize_opt {
            ($field:expr, $map:expr, $key:expr) => {
                if let Some(value) = &$field {
                    $map.serialize_entry($key, value)?;
                }
            };
        }

        macro_rules! serialize_flag {
            ($field:expr, $map:expr, $key:expr) => {
                if let Some(true) = &$field {
                    $map.serialize_entry($key, &1)?;
                }
            };
        }

        let mut map = serializer.serialize_map(None)?;

        serialize_opt!(self.title, map, "title");
        serialize_opt!(self.subtitle, map, "subtitle");

        if let Some(body) = &self.body {
            match self.body_type {
                BodyType::Markdown => map.serialize_entry("markdown", body)?,
                BodyType::Plaintext => map.serialize_entry("body", body)?,
            }
        }
        map.serialize_entry("device_keys", &self.device_keys)?;
        serialize_opt!(self.level, map, "level");
        serialize_opt!(self.volume, map, "volume");
        serialize_opt!(self.badge, map, "badge");
        serialize_flag!(self.call, map, "call");
        serialize_flag!(self.auto_copy, map, "autoCopy");
        serialize_opt!(self.copy, map, "copy");
        serialize_opt!(self.sound, map, "sound");
        serialize_opt!(self.icon, map, "icon");
        serialize_opt!(self.image, map, "image");
        serialize_opt!(self.group, map, "group");
        serialize_opt!(self.ciphertext, map, "ciphertext");
        serialize_flag!(self.is_archive, map, "isArchive");
        serialize_opt!(self.url, map, "url");
        serialize_opt!(self.action, map, "action");
        serialize_opt!(self.id, map, "id");
        serialize_flag!(self.delete, map, "delete");

        map.end()
    }
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
    body_type: BodyType,
    device_keys: Vec<String>,
    level: Option<Level>,
    volume: Option<u8>,
    badge: Option<u64>,
    call: Option<bool>,
    auto_copy: Option<bool>,
    copy: Option<String>,
    sound: Option<String>,
    icon: Option<String>,
    image: Option<String>,
    group: Option<String>,
    ciphertext: Option<String>,
    is_archive: Option<bool>,
    url: Option<String>,
    action: Option<String>,
    id: Option<String>,
    delete: Option<bool>,
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
    pub fn body_type(mut self, body_type: BodyType) -> Self {
        self.body_type = body_type;
        self
    }

    #[must_use]
    pub fn device_key(mut self, device_key: impl Into<String>) -> Self {
        self.device_keys.push(device_key.into());
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
        self.call = Some(call);
        self
    }

    #[must_use]
    pub fn auto_copy(mut self, auto_copy: bool) -> Self {
        self.auto_copy = Some(auto_copy);
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
        self.is_archive = Some(is_archive);
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
        self.delete = Some(delete);
        self
    }

    pub fn build(mut self) -> Result<Message> {
        if let Some(volume) = self.volume.filter(|&volume| volume > Self::MAX_VOLUME) {
            return Err(Error::VolumeOutOfRange {
                current: volume,
                max: Self::MAX_VOLUME,
            });
        }

        self.device_keys.dedup();

        Ok(Message {
            title: self.title,
            subtitle: self.subtitle,
            body: self.body,
            body_type: self.body_type,
            device_keys: self.device_keys,
            level: self.level,
            volume: self.volume,
            badge: self.badge,
            call: self.call,
            auto_copy: self.auto_copy,
            copy: self.copy,
            sound: self.sound,
            icon: self.icon,
            image: self.image,
            group: self.group,
            ciphertext: self.ciphertext,
            is_archive: self.is_archive,
            url: self.url,
            action: self.action,
            id: self.id,
            delete: self.delete,
        })
    }
}
