// Generated from definition io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.ExternalAdmissionHook

/// ExternalAdmissionHook describes an external admission webhook and the resources and operations it applies to.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ExternalAdmissionHook {
    /// ClientConfig defines how to communicate with the hook. Required
    #[serde(rename = "clientConfig")]
    pub client_config: ::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::AdmissionHookClientConfig,

    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Ignore.
    #[serde(rename = "failurePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<String>,

    /// The name of the external admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where "imagepolicy" is the name of the webhook, and kubernetes.io is the name of the organization. Required.
    pub name: String,

    /// Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::RuleWithOperations>>,
}
