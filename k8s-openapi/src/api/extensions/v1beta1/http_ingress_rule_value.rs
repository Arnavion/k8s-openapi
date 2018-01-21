// Generated from definition io.k8s.api.extensions.v1beta1.HTTPIngressRuleValue

/// HTTPIngressRuleValue is a list of http selectors pointing to backends. In the example: http://<host>/<path>?<searchpart> -> backend where where parts of the url correspond to RFC 3986, this resource will be used to match against everything after the last '/' and before the first '?' or '#'.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HTTPIngressRuleValue {
    /// A collection of paths that map requests to backends.
    pub paths: Vec<::api::extensions::v1beta1::HTTPIngressPath>,
}
