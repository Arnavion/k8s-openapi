/// The `"info"` property of an OpenAPI spec.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Info {
    pub title: String,
    pub version: String,
}
