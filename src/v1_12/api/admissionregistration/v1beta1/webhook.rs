// Generated from definition io.k8s.api.admissionregistration.v1beta1.Webhook

/// Webhook describes an admission webhook and the resources and operations it applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Webhook {
    /// ClientConfig defines how to communicate with the hook. Required
    pub client_config: crate::api::admissionregistration::v1beta1::WebhookClientConfig,

    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Ignore.
    pub failure_policy: Option<String>,

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
    /// See https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ for more examples of label selectors.
    ///
    /// Default to the empty LabelSelector, which matches everything.
    pub namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule. However, in order to prevent ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks from putting the cluster in a state which cannot be recovered from without completely disabling the plugin, ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks are never called on admission requests for ValidatingWebhookConfiguration and MutatingWebhookConfiguration objects.
    pub rules: Option<Vec<crate::api::admissionregistration::v1beta1::RuleWithOperations>>,

    /// SideEffects states whether this webhookk has side effects. Acceptable values are: Unknown, None, Some, NoneOnDryRun Webhooks with side effects MUST implement a reconciliation system, since a request may be rejected by a future step in the admission change and the side effects therefore need to be undone. Requests with the dryRun attribute will be auto-rejected if they match a webhook with sideEffects == Unknown or Some. Defaults to Unknown.
    pub side_effects: Option<String>,
}

impl<'de> serde::Deserialize<'de> for Webhook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_client_config,
            Key_failure_policy,
            Key_name,
            Key_namespace_selector,
            Key_rules,
            Key_side_effects,
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
                            "clientConfig" => Field::Key_client_config,
                            "failurePolicy" => Field::Key_failure_policy,
                            "name" => Field::Key_name,
                            "namespaceSelector" => Field::Key_namespace_selector,
                            "rules" => Field::Key_rules,
                            "sideEffects" => Field::Key_side_effects,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Webhook;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Webhook")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_client_config: Option<crate::api::admissionregistration::v1beta1::WebhookClientConfig> = None;
                let mut value_failure_policy: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_rules: Option<Vec<crate::api::admissionregistration::v1beta1::RuleWithOperations>> = None;
                let mut value_side_effects: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_client_config => value_client_config = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_failure_policy => value_failure_policy = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_namespace_selector => value_namespace_selector = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_side_effects => value_side_effects = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Webhook {
                    client_config: value_client_config.ok_or_else(|| serde::de::Error::missing_field("clientConfig"))?,
                    failure_policy: value_failure_policy,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    namespace_selector: value_namespace_selector,
                    rules: value_rules,
                    side_effects: value_side_effects,
                })
            }
        }

        deserializer.deserialize_struct(
            "Webhook",
            &[
                "clientConfig",
                "failurePolicy",
                "name",
                "namespaceSelector",
                "rules",
                "sideEffects",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for Webhook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Webhook",
            2 +
            self.failure_policy.as_ref().map_or(0, |_| 1) +
            self.namespace_selector.as_ref().map_or(0, |_| 1) +
            self.rules.as_ref().map_or(0, |_| 1) +
            self.side_effects.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "clientConfig", &self.client_config)?;
        if let Some(value) = &self.failure_policy {
            serde::ser::SerializeStruct::serialize_field(&mut state, "failurePolicy", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.namespace_selector {
            serde::ser::SerializeStruct::serialize_field(&mut state, "namespaceSelector", value)?;
        }
        if let Some(value) = &self.rules {
            serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        if let Some(value) = &self.side_effects {
            serde::ser::SerializeStruct::serialize_field(&mut state, "sideEffects", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
