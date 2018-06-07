// Generated from definition io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.Initializer

/// Initializer describes the name and the failure policy of an initializer, and what resources it applies to.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Initializer {
    /// FailurePolicy defines what happens if the responsible initializer controller fails to takes action. Allowed values are Ignore, or Fail. If "Ignore" is set, initializer is removed from the initializers list of an object if the timeout is reached; If "Fail" is set, admissionregistration returns timeout error if the timeout is reached.
    #[serde(rename = "failurePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<String>,

    /// Name is the identifier of the initializer. It will be added to the object that needs to be initialized. Name should be fully qualified, e.g., alwayspullimages.kubernetes.io, where "alwayspullimages" is the name of the webhook, and kubernetes.io is the name of the organization. Required
    pub name: String,

    /// Rules describes what resources/subresources the initializer cares about. The initializer cares about an operation if it matches _any_ Rule. Rule.Resources must not include subresources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<::v1_7::kubernetes::pkg::apis::admissionregistration::v1alpha1::Rule>>,
}
