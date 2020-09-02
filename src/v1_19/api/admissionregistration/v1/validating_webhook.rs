// Generated from definition io.k8s.api.admissionregistration.v1.ValidatingWebhook

/// ValidatingWebhook describes an admission webhook and the resources and operations it applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidatingWebhook {
    /// AdmissionReviewVersions is an ordered list of preferred `AdmissionReview` versions the Webhook expects. API server will try to use first version in the list which it supports. If none of the versions specified in this list supported by API server, validation will fail for this object. If a persisted webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail and be subject to the failure policy.
    pub admission_review_versions: Vec<String>,

    /// ClientConfig defines how to communicate with the hook. Required
    pub client_config: crate::api::admissionregistration::v1::WebhookClientConfig,

    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Fail.
    pub failure_policy: Option<String>,

    /// matchPolicy defines how the "rules" list is used to match incoming requests. Allowed values are "Exact" or "Equivalent".
    ///
    /// - Exact: match a request only if it exactly matches a specified rule. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, but "rules" only included `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]`, a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the webhook.
    ///
    /// - Equivalent: match a request if modifies a resource listed in rules, even via another API group or version. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, and "rules" only included `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]`, a request to apps/v1beta1 or extensions/v1beta1 would be converted to apps/v1 and sent to the webhook.
    ///
    /// Defaults to "Equivalent"
    pub match_policy: Option<String>,

    /// The name of the admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where "imagepolicy" is the name of the webhook, and kubernetes.io is the name of the organization. Required.
    pub name: String,

    /// NamespaceSelector decides whether to run the webhook on an object based on whether the namespace for that object matches the selector. If the object itself is a namespace, the matching is performed on object.metadata.labels. If the object is another cluster scoped resource, it never skips the webhook.
    ///
    /// For example, to run the webhook on any objects whose namespace is not associated with "runlevel" of "0" or "1";  you will set the selector as follows: "namespaceSelector": {
    ///   "matchExpressions": \[
    ///     {
    ///       "key": "runlevel",
    ///       "operator": "NotIn",
    ///       "values": \[
    ///         "0",
    ///         "1"
    ///       \]
    ///     }
    ///   \]
    /// }
    ///
    /// If instead you want to only run the webhook on any objects whose namespace is associated with the "environment" of "prod" or "staging"; you will set the selector as follows: "namespaceSelector": {
    ///   "matchExpressions": \[
    ///     {
    ///       "key": "environment",
    ///       "operator": "In",
    ///       "values": \[
    ///         "prod",
    ///         "staging"
    ///       \]
    ///     }
    ///   \]
    /// }
    ///
    /// See https://kubernetes.io/docs/concepts/overview/working-with-objects/labels for more examples of label selectors.
    ///
    /// Default to the empty LabelSelector, which matches everything.
    pub namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// ObjectSelector decides whether to run the webhook based on if the object has matching labels. objectSelector is evaluated against both the oldObject and newObject that would be sent to the webhook, and is considered to match if either object matches the selector. A null object (oldObject in the case of create, or newObject in the case of delete) or an object that cannot have labels (like a DeploymentRollback or a PodProxyOptions object) is not considered to match. Use the object selector only if the webhook is opt-in, because end users may skip the admission webhook by setting the labels. Default to the empty LabelSelector, which matches everything.
    pub object_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule. However, in order to prevent ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks from putting the cluster in a state which cannot be recovered from without completely disabling the plugin, ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks are never called on admission requests for ValidatingWebhookConfiguration and MutatingWebhookConfiguration objects.
    pub rules: Option<Vec<crate::api::admissionregistration::v1::RuleWithOperations>>,

    /// SideEffects states whether this webhook has side effects. Acceptable values are: None, NoneOnDryRun (webhooks created via v1beta1 may also specify Some or Unknown). Webhooks with side effects MUST implement a reconciliation system, since a request may be rejected by a future step in the admission change and the side effects therefore need to be undone. Requests with the dryRun attribute will be auto-rejected if they match a webhook with sideEffects == Unknown or Some.
    pub side_effects: String,

    /// TimeoutSeconds specifies the timeout for this webhook. After the timeout passes, the webhook call will be ignored or the API call will fail based on the failure policy. The timeout value must be between 1 and 30 seconds. Default to 10 seconds.
    pub timeout_seconds: Option<i32>,
}

impl<'de> serde::Deserialize<'de> for ValidatingWebhook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_admission_review_versions,
            Key_client_config,
            Key_failure_policy,
            Key_match_policy,
            Key_name,
            Key_namespace_selector,
            Key_object_selector,
            Key_rules,
            Key_side_effects,
            Key_timeout_seconds,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "admissionReviewVersions" => Field::Key_admission_review_versions,
                            "clientConfig" => Field::Key_client_config,
                            "failurePolicy" => Field::Key_failure_policy,
                            "matchPolicy" => Field::Key_match_policy,
                            "name" => Field::Key_name,
                            "namespaceSelector" => Field::Key_namespace_selector,
                            "objectSelector" => Field::Key_object_selector,
                            "rules" => Field::Key_rules,
                            "sideEffects" => Field::Key_side_effects,
                            "timeoutSeconds" => Field::Key_timeout_seconds,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ValidatingWebhook;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ValidatingWebhook")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_admission_review_versions: Option<Vec<String>> = None;
                let mut value_client_config: Option<crate::api::admissionregistration::v1::WebhookClientConfig> = None;
                let mut value_failure_policy: Option<String> = None;
                let mut value_match_policy: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_object_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_rules: Option<Vec<crate::api::admissionregistration::v1::RuleWithOperations>> = None;
                let mut value_side_effects: Option<String> = None;
                let mut value_timeout_seconds: Option<i32> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_admission_review_versions => value_admission_review_versions = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_client_config => value_client_config = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_failure_policy => value_failure_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_policy => value_match_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_namespace_selector => value_namespace_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_object_selector => value_object_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_side_effects => value_side_effects = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_timeout_seconds => value_timeout_seconds = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ValidatingWebhook {
                    admission_review_versions: value_admission_review_versions.ok_or_else(|| serde::de::Error::missing_field("admissionReviewVersions"))?,
                    client_config: value_client_config.ok_or_else(|| serde::de::Error::missing_field("clientConfig"))?,
                    failure_policy: value_failure_policy,
                    match_policy: value_match_policy,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    namespace_selector: value_namespace_selector,
                    object_selector: value_object_selector,
                    rules: value_rules,
                    side_effects: value_side_effects.ok_or_else(|| serde::de::Error::missing_field("sideEffects"))?,
                    timeout_seconds: value_timeout_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "ValidatingWebhook",
            &[
                "admissionReviewVersions",
                "clientConfig",
                "failurePolicy",
                "matchPolicy",
                "name",
                "namespaceSelector",
                "objectSelector",
                "rules",
                "sideEffects",
                "timeoutSeconds",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ValidatingWebhook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ValidatingWebhook",
            4 +
            self.failure_policy.as_ref().map_or(0, |_| 1) +
            self.match_policy.as_ref().map_or(0, |_| 1) +
            self.namespace_selector.as_ref().map_or(0, |_| 1) +
            self.object_selector.as_ref().map_or(0, |_| 1) +
            self.rules.as_ref().map_or(0, |_| 1) +
            self.timeout_seconds.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "admissionReviewVersions", &self.admission_review_versions)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "clientConfig", &self.client_config)?;
        if let Some(value) = &self.failure_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "failurePolicy", value)?;
        }
        if let Some(value) = &self.match_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "matchPolicy", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.namespace_selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "namespaceSelector", value)?;
        }
        if let Some(value) = &self.object_selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "objectSelector", value)?;
        }
        if let Some(value) = &self.rules {
            serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "sideEffects", &self.side_effects)?;
        if let Some(value) = &self.timeout_seconds {
            serde::ser::SerializeStruct::serialize_field(&mut state, "timeoutSeconds", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
