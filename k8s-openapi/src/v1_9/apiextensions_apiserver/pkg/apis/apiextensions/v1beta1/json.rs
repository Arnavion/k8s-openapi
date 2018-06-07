// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSON

/// JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JSON {
    #[serde(rename = "Raw")]
    pub raw: ::ByteString,
}
