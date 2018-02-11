extern crate base64;
extern crate chrono;
extern crate serde;
#[macro_use] extern crate serde_derive;

pub mod api;

pub mod apiextensions_apiserver;

pub mod apimachinery;

pub mod kube_aggregator;

#[derive(Debug, Default)]
pub struct ByteString(pub Vec<u8>);

impl serde::Serialize for ByteString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer {
        base64::encode_config(&self.0, base64::STANDARD).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ByteString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Ok(ByteString(base64::decode_config(&s, base64::STANDARD).map_err(serde::de::Error::custom)?))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IntOrString {
    Int(i32),
    String(String),
}

impl Default for IntOrString {
    fn default() -> Self {
        IntOrString::Int(0)
    }
}
