use std::{collections::BTreeSet, fmt, marker::PhantomData};

use serde::{
    Deserialize, Deserializer, de::MapAccess, de::Visitor, de::value::MapAccessDeserializer,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct DeviceResult {
    pub device_key: String,
    pub code: i64,
    pub message: Option<String>,
}

impl DeviceResult {
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.code == 200
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PushResponse {
    pub code: i64,
    pub message: String,
    pub timestamp: i64,

    #[serde(rename = "data", default, deserialize_with = "deserialize_results")]
    pub results: Option<Vec<DeviceResult>>,
}

impl PushResponse {
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.code == 200
            && self.results.as_ref().is_none_or(|results| {
                !results.is_empty() && results.iter().all(DeviceResult::is_success)
            })
    }

    pub(crate) fn matches_recipients(&self, keys: &BTreeSet<String>) -> bool {
        match &self.results {
            None => keys.len() == 1,
            Some(results) => {
                keys.len() > 1
                    && results.len() == keys.len()
                    && results
                        .iter()
                        .map(|result| &result.device_key)
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .eq(keys.iter())
            }
        }
    }
}

pub(crate) fn decode_response(body: &[u8]) -> serde_json::Result<PushResponse> {
    let mut deserializer = serde_json::Deserializer::from_slice(body);
    let response = deserialize_object(&mut deserializer)?;
    deserializer.end()?;

    Ok(response)
}

fn deserialize_results<'de, D>(deserializer: D) -> Result<Option<Vec<DeviceResult>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Object(#[serde(deserialize_with = "deserialize_object")] DeviceResult);

    Option::<Vec<Object>>::deserialize(deserializer).map(|results| {
        results.map(|results| results.into_iter().map(|Object(result)| result).collect())
    })
}

fn deserialize_object<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct ObjectVisitor<T>(PhantomData<T>);

    impl<'de, T: Deserialize<'de>> Visitor<'de> for ObjectVisitor<T> {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a JSON object")
        }

        fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<T, A::Error> {
            T::deserialize(MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_map(ObjectVisitor(PhantomData))
}
