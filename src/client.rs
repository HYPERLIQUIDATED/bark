use std::{borrow::Cow, collections::BTreeSet, time::Duration};

use reqwest::Url;

use crate::{
    error::{Error, Result},
    message::{Message, validate_device_keys},
    response::{PushResponse, decode_response},
};

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    push_url: Url,
    default_device_keys: BTreeSet<String>,
}

impl Client {
    pub const DEFAULT_SERVER: &str = "https://api.day.app";

    /// # Errors
    /// Returns an error if the server URL is invalid or the HTTP client cannot be built.
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .build()?;

        Self::with_http_client(base_url, http)
    }

    /// # Errors
    /// Returns an error unless the URL is absolute HTTP(S) without a query or fragment.
    pub fn with_http_client(base_url: impl AsRef<str>, http: reqwest::Client) -> Result<Self> {
        Ok(Self {
            http,
            push_url: parse_push_url(base_url.as_ref())?,
            default_device_keys: BTreeSet::new(),
        })
    }

    #[must_use]
    pub fn with_device_key(mut self, device_key: impl Into<String>) -> Self {
        self.default_device_keys.insert(device_key.into());
        self
    }

    #[must_use]
    pub fn with_device_keys(
        mut self,
        device_keys: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.default_device_keys
            .extend(device_keys.into_iter().map(Into::into));
        self
    }

    /// # Errors
    /// Returns an error for invalid recipients, transport or response failures,
    /// or rejected pushes, including partial batch failures.
    pub async fn send(&self, message: &Message) -> Result<PushResponse> {
        let mut payload = Cow::Borrowed(message);

        if payload.device_keys.is_empty() {
            payload
                .to_mut()
                .device_keys
                .clone_from(&self.default_device_keys);
        }

        if payload.device_keys.is_empty() {
            return Err(Error::MissingDeviceKey);
        }

        validate_device_keys(&payload.device_keys)?;

        let response = self
            .http
            .post(self.push_url.clone())
            .json(payload.as_ref())
            .send()
            .await?;
        let status = response.status();
        let body = response.bytes().await?;
        let parsed = decode_response(&body);

        if !status.is_success() {
            return Err(match parsed {
                Ok(response) => Error::Api { status, response },
                Err(_) => Error::Http {
                    status,
                    body: String::from_utf8_lossy(&body).into_owned(),
                },
            });
        }

        let response = parsed?;

        if response.code != 200 {
            return Err(Error::Api { status, response });
        }

        if !response.matches_recipients(&payload.device_keys) {
            return Err(Error::UnexpectedResponse { response });
        }

        if !response.is_success() {
            return Err(Error::Batch { response });
        }

        Ok(response)
    }
}

fn parse_push_url(base_url: &str) -> Result<Url> {
    let mut url = Url::parse(base_url).map_err(|err| Error::InvalidBaseUrl(err.to_string()))?;

    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err(Error::InvalidBaseUrl(
            "expected an absolute HTTP(S) URL".into(),
        ));
    }

    if url.query().is_some() || url.fragment().is_some() {
        return Err(Error::InvalidBaseUrl(
            "query strings and fragments are not supported".into(),
        ));
    }

    url.set_path(&format!("{}/push", url.path().trim_end_matches('/')));

    Ok(url)
}
