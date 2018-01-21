// Generated from definition io.k8s.api.admissionregistration.v1alpha1.Initializer

/// Initializer describes the name and the failure policy of an initializer, and what resources it applies to.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Initializer {
    /// Name is the identifier of the initializer. It will be added to the object that needs to be initialized. Name should be fully qualified, e.g., alwayspullimages.kubernetes.io, where "alwayspullimages" is the name of the webhook, and kubernetes.io is the name of the organization. Required
    pub name: String,

    /// Rules describes what resources/subresources the initializer cares about. The initializer cares about an operation if it matches _any_ Rule. Rule.Resources must not include subresources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<::api::admissionregistration::v1alpha1::Rule>>,
}
