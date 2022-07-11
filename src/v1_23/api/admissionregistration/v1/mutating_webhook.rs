// Generated from definition io.k8s.api.admissionregistration.v1.MutatingWebhook

/// MutatingWebhook describes an admission webhook and the resources and operations it applies to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MutatingWebhook {
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
    /// See https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ for more examples of label selectors.
    ///
    /// Default to the empty LabelSelector, which matches everything.
    pub namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// ObjectSelector decides whether to run the webhook based on if the object has matching labels. objectSelector is evaluated against both the oldObject and newObject that would be sent to the webhook, and is considered to match if either object matches the selector. A null object (oldObject in the case of create, or newObject in the case of delete) or an object that cannot have labels (like a DeploymentRollback or a PodProxyOptions object) is not considered to match. Use the object selector only if the webhook is opt-in, because end users may skip the admission webhook by setting the labels. Default to the empty LabelSelector, which matches everything.
    pub object_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,

    /// reinvocationPolicy indicates whether this webhook should be called multiple times as part of a single admission evaluation. Allowed values are "Never" and "IfNeeded".
    ///
    /// Never: the webhook will not be called more than once in a single admission evaluation.
    ///
    /// IfNeeded: the webhook will be called at least one additional time as part of the admission evaluation if the object being admitted is modified by other admission plugins after the initial webhook call. Webhooks that specify this option *must* be idempotent, able to process objects they previously admitted. Note: * the number of additional invocations is not guaranteed to be exactly one. * if additional invocations result in further modifications to the object, webhooks are not guaranteed to be invoked again. * webhooks that use this option may be reordered to minimize the number of additional invocations. * to validate an object after all mutations are guaranteed complete, use a validating admission webhook instead.
    ///
    /// Defaults to "Never".
    pub reinvocation_policy: Option<String>,

    /// Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule. However, in order to prevent ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks from putting the cluster in a state which cannot be recovered from without completely disabling the plugin, ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks are never called on admission requests for ValidatingWebhookConfiguration and MutatingWebhookConfiguration objects.
    pub rules: Option<Vec<crate::api::admissionregistration::v1::RuleWithOperations>>,

    /// SideEffects states whether this webhook has side effects. Acceptable values are: None, NoneOnDryRun (webhooks created via v1beta1 may also specify Some or Unknown). Webhooks with side effects MUST implement a reconciliation system, since a request may be rejected by a future step in the admission chain and the side effects therefore need to be undone. Requests with the dryRun attribute will be auto-rejected if they match a webhook with sideEffects == Unknown or Some.
    pub side_effects: String,

    /// TimeoutSeconds specifies the timeout for this webhook. After the timeout passes, the webhook call will be ignored or the API call will fail based on the failure policy. The timeout value must be between 1 and 30 seconds. Default to 10 seconds.
    pub timeout_seconds: Option<i32>,

}

#[cfg(feature = "dsl")]
impl MutatingWebhook  {
    /// Set [`Self::admission_review_versions`]
    pub  fn admission_review_versions_set(&mut self, admission_review_versions: impl Into<Vec<String>>) -> &mut Self {
        self.admission_review_versions = admission_review_versions.into(); self
    }

    pub  fn admission_review_versions(&mut self) -> &mut Vec<String> {
        &mut self.admission_review_versions
    }

    /// Modify [`Self::admission_review_versions`] with a `func`
    pub  fn admission_review_versions_with(&mut self, func: impl FnOnce(&mut Vec<String>)) -> &mut Self {
        func(&mut self.admission_review_versions); self
    }

    /// Push new element to [`Self::admission_review_versions`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn admission_review_versions_push_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
      let mut new = Default::default();
      func(&mut new);
      self.admission_review_versions.push(new);
      self
    }

    /// Append all elements from `other` into [`Self::admission_review_versions`]
    pub  fn admission_review_versions_append_from(&mut self, other: impl std::borrow::Borrow<[String]>) -> &mut Self {
         for item in other.borrow() {
             self.admission_review_versions.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::client_config`]
    pub  fn client_config_set(&mut self, client_config: impl Into<crate::api::admissionregistration::v1::WebhookClientConfig>) -> &mut Self {
        self.client_config = client_config.into(); self
    }

    pub  fn client_config(&mut self) -> &mut crate::api::admissionregistration::v1::WebhookClientConfig {
        &mut self.client_config
    }

    /// Modify [`Self::client_config`] with a `func`
    pub  fn client_config_with(&mut self, func: impl FnOnce(&mut crate::api::admissionregistration::v1::WebhookClientConfig)) -> &mut Self {
        func(&mut self.client_config); self
    }


    /// Set [`Self::failure_policy`]
    pub  fn failure_policy_set(&mut self, failure_policy: impl Into<Option<String>>) -> &mut Self {
        self.failure_policy = failure_policy.into(); self
    }

    pub  fn failure_policy(&mut self) -> &mut String {
        if self.failure_policy.is_none() { self.failure_policy = Some(Default::default()) }
        self.failure_policy.as_mut().unwrap()
    }

    /// Modify [`Self::failure_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn failure_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.failure_policy.is_none() { self.failure_policy = Some(Default::default()) };
        func(self.failure_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::match_policy`]
    pub  fn match_policy_set(&mut self, match_policy: impl Into<Option<String>>) -> &mut Self {
        self.match_policy = match_policy.into(); self
    }

    pub  fn match_policy(&mut self) -> &mut String {
        if self.match_policy.is_none() { self.match_policy = Some(Default::default()) }
        self.match_policy.as_mut().unwrap()
    }

    /// Modify [`Self::match_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn match_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.match_policy.is_none() { self.match_policy = Some(Default::default()) };
        func(self.match_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::name`]
    pub  fn name_set(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into(); self
    }

    pub  fn name(&mut self) -> &mut String {
        &mut self.name
    }

    /// Modify [`Self::name`] with a `func`
    pub  fn name_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.name); self
    }


    /// Set [`Self::namespace_selector`]
    pub  fn namespace_selector_set(&mut self, namespace_selector: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>>) -> &mut Self {
        self.namespace_selector = namespace_selector.into(); self
    }

    pub  fn namespace_selector(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector {
        if self.namespace_selector.is_none() { self.namespace_selector = Some(Default::default()) }
        self.namespace_selector.as_mut().unwrap()
    }

    /// Modify [`Self::namespace_selector`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn namespace_selector_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector)) -> &mut Self {
        if self.namespace_selector.is_none() { self.namespace_selector = Some(Default::default()) };
        func(self.namespace_selector.as_mut().unwrap()); self
    }


    /// Set [`Self::object_selector`]
    pub  fn object_selector_set(&mut self, object_selector: impl Into<Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>>) -> &mut Self {
        self.object_selector = object_selector.into(); self
    }

    pub  fn object_selector(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector {
        if self.object_selector.is_none() { self.object_selector = Some(Default::default()) }
        self.object_selector.as_mut().unwrap()
    }

    /// Modify [`Self::object_selector`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn object_selector_with(&mut self, func: impl FnOnce(&mut crate::apimachinery::pkg::apis::meta::v1::LabelSelector)) -> &mut Self {
        if self.object_selector.is_none() { self.object_selector = Some(Default::default()) };
        func(self.object_selector.as_mut().unwrap()); self
    }


    /// Set [`Self::reinvocation_policy`]
    pub  fn reinvocation_policy_set(&mut self, reinvocation_policy: impl Into<Option<String>>) -> &mut Self {
        self.reinvocation_policy = reinvocation_policy.into(); self
    }

    pub  fn reinvocation_policy(&mut self) -> &mut String {
        if self.reinvocation_policy.is_none() { self.reinvocation_policy = Some(Default::default()) }
        self.reinvocation_policy.as_mut().unwrap()
    }

    /// Modify [`Self::reinvocation_policy`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn reinvocation_policy_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        if self.reinvocation_policy.is_none() { self.reinvocation_policy = Some(Default::default()) };
        func(self.reinvocation_policy.as_mut().unwrap()); self
    }


    /// Set [`Self::rules`]
    pub  fn rules_set(&mut self, rules: impl Into<Option<Vec<crate::api::admissionregistration::v1::RuleWithOperations>>>) -> &mut Self {
        self.rules = rules.into(); self
    }

    pub  fn rules(&mut self) -> &mut Vec<crate::api::admissionregistration::v1::RuleWithOperations> {
        if self.rules.is_none() { self.rules = Some(Default::default()) }
        self.rules.as_mut().unwrap()
    }

    /// Modify [`Self::rules`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn rules_with(&mut self, func: impl FnOnce(&mut Vec<crate::api::admissionregistration::v1::RuleWithOperations>)) -> &mut Self {
        if self.rules.is_none() { self.rules = Some(Default::default()) };
        func(self.rules.as_mut().unwrap()); self
    }

    /// Push new element to [`Self::rules`] and modify with a `func`
    ///
    /// The field will initially set to `Default::default()`
    pub  fn rules_push_with(&mut self, func: impl FnOnce(&mut crate::api::admissionregistration::v1::RuleWithOperations)) -> &mut Self {
        if self.rules.is_none() {
            self.rules = Some(vec![]);
        }
        let mut new = Default::default();
        func(&mut new);
        self.rules.as_mut().unwrap().push(new);
        self
    }

    /// Append all elements from `other` into [`Self::rules`]
    pub  fn rules_append_from(&mut self, other: impl std::borrow::Borrow<[crate::api::admissionregistration::v1::RuleWithOperations]>) -> &mut Self {
         if self.rules.is_none() { self.rules = Some(Vec::new()); }
         let rules = &mut self.rules.as_mut().unwrap();
         for item in other.borrow() {
             rules.push(item.to_owned());
         }
         self
    }


    /// Set [`Self::side_effects`]
    pub  fn side_effects_set(&mut self, side_effects: impl Into<String>) -> &mut Self {
        self.side_effects = side_effects.into(); self
    }

    pub  fn side_effects(&mut self) -> &mut String {
        &mut self.side_effects
    }

    /// Modify [`Self::side_effects`] with a `func`
    pub  fn side_effects_with(&mut self, func: impl FnOnce(&mut String)) -> &mut Self {
        func(&mut self.side_effects); self
    }


    /// Set [`Self::timeout_seconds`]
    pub  fn timeout_seconds_set(&mut self, timeout_seconds: impl Into<Option<i32>>) -> &mut Self {
        self.timeout_seconds = timeout_seconds.into(); self
    }

    pub  fn timeout_seconds(&mut self) -> &mut i32 {
        if self.timeout_seconds.is_none() { self.timeout_seconds = Some(Default::default()) }
        self.timeout_seconds.as_mut().unwrap()
    }

    /// Modify [`Self::timeout_seconds`] with a `func`
    ///
    /// The field will be set to `Default::default()` if not set before
    pub  fn timeout_seconds_with(&mut self, func: impl FnOnce(&mut i32)) -> &mut Self {
        if self.timeout_seconds.is_none() { self.timeout_seconds = Some(Default::default()) };
        func(self.timeout_seconds.as_mut().unwrap()); self
    }


}


impl<'de> crate::serde::Deserialize<'de> for MutatingWebhook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_admission_review_versions,
            Key_client_config,
            Key_failure_policy,
            Key_match_policy,
            Key_name,
            Key_namespace_selector,
            Key_object_selector,
            Key_reinvocation_policy,
            Key_rules,
            Key_side_effects,
            Key_timeout_seconds,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "admissionReviewVersions" => Field::Key_admission_review_versions,
                            "clientConfig" => Field::Key_client_config,
                            "failurePolicy" => Field::Key_failure_policy,
                            "matchPolicy" => Field::Key_match_policy,
                            "name" => Field::Key_name,
                            "namespaceSelector" => Field::Key_namespace_selector,
                            "objectSelector" => Field::Key_object_selector,
                            "reinvocationPolicy" => Field::Key_reinvocation_policy,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = MutatingWebhook;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("MutatingWebhook")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_admission_review_versions: Option<Vec<String>> = None;
                let mut value_client_config: Option<crate::api::admissionregistration::v1::WebhookClientConfig> = None;
                let mut value_failure_policy: Option<String> = None;
                let mut value_match_policy: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_object_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;
                let mut value_reinvocation_policy: Option<String> = None;
                let mut value_rules: Option<Vec<crate::api::admissionregistration::v1::RuleWithOperations>> = None;
                let mut value_side_effects: Option<String> = None;
                let mut value_timeout_seconds: Option<i32> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_admission_review_versions => value_admission_review_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_client_config => value_client_config = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failure_policy => value_failure_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_policy => value_match_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace_selector => value_namespace_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_object_selector => value_object_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reinvocation_policy => value_reinvocation_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_side_effects => value_side_effects = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_timeout_seconds => value_timeout_seconds = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MutatingWebhook {
                    admission_review_versions: value_admission_review_versions.unwrap_or_default(),
                    client_config: value_client_config.unwrap_or_default(),
                    failure_policy: value_failure_policy,
                    match_policy: value_match_policy,
                    name: value_name.unwrap_or_default(),
                    namespace_selector: value_namespace_selector,
                    object_selector: value_object_selector,
                    reinvocation_policy: value_reinvocation_policy,
                    rules: value_rules,
                    side_effects: value_side_effects.unwrap_or_default(),
                    timeout_seconds: value_timeout_seconds,
                })
            }
        }

        deserializer.deserialize_struct(
            "MutatingWebhook",
            &[
                "admissionReviewVersions",
                "clientConfig",
                "failurePolicy",
                "matchPolicy",
                "name",
                "namespaceSelector",
                "objectSelector",
                "reinvocationPolicy",
                "rules",
                "sideEffects",
                "timeoutSeconds",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for MutatingWebhook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MutatingWebhook",
            4 +
            self.failure_policy.as_ref().map_or(0, |_| 1) +
            self.match_policy.as_ref().map_or(0, |_| 1) +
            self.namespace_selector.as_ref().map_or(0, |_| 1) +
            self.object_selector.as_ref().map_or(0, |_| 1) +
            self.reinvocation_policy.as_ref().map_or(0, |_| 1) +
            self.rules.as_ref().map_or(0, |_| 1) +
            self.timeout_seconds.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "admissionReviewVersions", &self.admission_review_versions)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "clientConfig", &self.client_config)?;
        if let Some(value) = &self.failure_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failurePolicy", value)?;
        }
        if let Some(value) = &self.match_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchPolicy", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.namespace_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespaceSelector", value)?;
        }
        if let Some(value) = &self.object_selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "objectSelector", value)?;
        }
        if let Some(value) = &self.reinvocation_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reinvocationPolicy", value)?;
        }
        if let Some(value) = &self.rules {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "sideEffects", &self.side_effects)?;
        if let Some(value) = &self.timeout_seconds {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "timeoutSeconds", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for MutatingWebhook {
    fn schema_name() -> String {
        "io.k8s.api.admissionregistration.v1.MutatingWebhook".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("MutatingWebhook describes an admission webhook and the resources and operations it applies to.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "admissionReviewVersions".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("AdmissionReviewVersions is an ordered list of preferred `AdmissionReview` versions the Webhook expects. API server will try to use first version in the list which it supports. If none of the versions specified in this list supported by API server, validation will fail for this object. If a persisted webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail and be subject to the failure policy.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "clientConfig".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1::WebhookClientConfig>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ClientConfig defines how to communicate with the hook. Required".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "failurePolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Fail.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "matchPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("matchPolicy defines how the \"rules\" list is used to match incoming requests. Allowed values are \"Exact\" or \"Equivalent\".\n\n- Exact: match a request only if it exactly matches a specified rule. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, but \"rules\" only included `apiGroups:[\"apps\"], apiVersions:[\"v1\"], resources: [\"deployments\"]`, a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the webhook.\n\n- Equivalent: match a request if modifies a resource listed in rules, even via another API group or version. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, and \"rules\" only included `apiGroups:[\"apps\"], apiVersions:[\"v1\"], resources: [\"deployments\"]`, a request to apps/v1beta1 or extensions/v1beta1 would be converted to apps/v1 and sent to the webhook.\n\nDefaults to \"Equivalent\"".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "name".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("The name of the admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where \"imagepolicy\" is the name of the webhook, and kubernetes.io is the name of the organization. Required.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namespaceSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("NamespaceSelector decides whether to run the webhook on an object based on whether the namespace for that object matches the selector. If the object itself is a namespace, the matching is performed on object.metadata.labels. If the object is another cluster scoped resource, it never skips the webhook.\n\nFor example, to run the webhook on any objects whose namespace is not associated with \"runlevel\" of \"0\" or \"1\";  you will set the selector as follows: \"namespaceSelector\": {\n  \"matchExpressions\": [\n    {\n      \"key\": \"runlevel\",\n      \"operator\": \"NotIn\",\n      \"values\": [\n        \"0\",\n        \"1\"\n      ]\n    }\n  ]\n}\n\nIf instead you want to only run the webhook on any objects whose namespace is associated with the \"environment\" of \"prod\" or \"staging\"; you will set the selector as follows: \"namespaceSelector\": {\n  \"matchExpressions\": [\n    {\n      \"key\": \"environment\",\n      \"operator\": \"In\",\n      \"values\": [\n        \"prod\",\n        \"staging\"\n      ]\n    }\n  ]\n}\n\nSee https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ for more examples of label selectors.\n\nDefault to the empty LabelSelector, which matches everything.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "objectSelector".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ObjectSelector decides whether to run the webhook based on if the object has matching labels. objectSelector is evaluated against both the oldObject and newObject that would be sent to the webhook, and is considered to match if either object matches the selector. A null object (oldObject in the case of create, or newObject in the case of delete) or an object that cannot have labels (like a DeploymentRollback or a PodProxyOptions object) is not considered to match. Use the object selector only if the webhook is opt-in, because end users may skip the admission webhook by setting the labels. Default to the empty LabelSelector, which matches everything.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "reinvocationPolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("reinvocationPolicy indicates whether this webhook should be called multiple times as part of a single admission evaluation. Allowed values are \"Never\" and \"IfNeeded\".\n\nNever: the webhook will not be called more than once in a single admission evaluation.\n\nIfNeeded: the webhook will be called at least one additional time as part of the admission evaluation if the object being admitted is modified by other admission plugins after the initial webhook call. Webhooks that specify this option *must* be idempotent, able to process objects they previously admitted. Note: * the number of additional invocations is not guaranteed to be exactly one. * if additional invocations result in further modifications to the object, webhooks are not guaranteed to be invoked again. * webhooks that use this option may be reordered to minimize the number of additional invocations. * to validate an object after all mutations are guaranteed complete, use a validating admission webhook instead.\n\nDefaults to \"Never\".".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "rules".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule. However, in order to prevent ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks from putting the cluster in a state which cannot be recovered from without completely disabling the plugin, ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks are never called on admission requests for ValidatingWebhookConfiguration and MutatingWebhookConfiguration objects.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::admissionregistration::v1::RuleWithOperations>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "sideEffects".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("SideEffects states whether this webhook has side effects. Acceptable values are: None, NoneOnDryRun (webhooks created via v1beta1 may also specify Some or Unknown). Webhooks with side effects MUST implement a reconciliation system, since a request may be rejected by a future step in the admission chain and the side effects therefore need to be undone. Requests with the dryRun attribute will be auto-rejected if they match a webhook with sideEffects == Unknown or Some.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "timeoutSeconds".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("TimeoutSeconds specifies the timeout for this webhook. After the timeout passes, the webhook call will be ignored or the API call will fail based on the failure policy. The timeout value must be between 1 and 30 seconds. Default to 10 seconds.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int32".to_owned()),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "admissionReviewVersions".to_owned(),
                    "clientConfig".to_owned(),
                    "name".to_owned(),
                    "sideEffects".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
