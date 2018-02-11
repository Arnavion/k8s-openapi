// Generated from definition io.k8s.api.admissionregistration.v1beta1.Webhook

/// Webhook describes an admission webhook and the resources and operations it applies to.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Webhook {
    /// ClientConfig defines how to communicate with the hook. Required
    #[serde(rename = "clientConfig")]
    pub client_config: ::api::admissionregistration::v1beta1::WebhookClientConfig,

    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Ignore.
    #[serde(rename = "failurePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<String>,

    /// The name of the admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where "imagepolicy" is the name of the webhook, and kubernetes.io is the name of the organization. Required.
    pub name: String,

    /// NamespaceSelector decides whether to run the webhook on an object based on whether the namespace for that object matches the selector. If the object itself is a namespace, the matching is performed on object.metadata.labels. If the object is another cluster scoped resource, it never skips the webhook.
    ///
    /// For example, to run the webhook on any objects whose namespace is not associated with "runlevel" of "0" or "1";  you will set the selector as follows: "namespaceSelector": {
    ///   "matchExpressions": [
    ///     {
    ///       "key": "runlevel",
    ///       "operator": "NotIn",
    ///       "values": [
    ///         "0",
    ///         "1"
    ///       ]
    ///     }
    ///   ]
    /// }
    ///
    /// If instead you want to only run the webhook on any objects whose namespace is associated with the "environment" of "prod" or "staging"; you will set the selector as follows: "namespaceSelector": {
    ///   "matchExpressions": [
    ///     {
    ///       "key": "environment",
    ///       "operator": "In",
    ///       "values": [
    ///         "prod",
    ///         "staging"
    ///       ]
    ///     }
    ///   ]
    /// }
    ///
    /// See https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ for more examples of label selectors.
    ///
    /// Default to the empty LabelSelector, which matches everything.
    #[serde(rename = "namespaceSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<::api::admissionregistration::v1beta1::RuleWithOperations>>,
}
