// Generated from definition io.k8s.api.admissionregistration.v1alpha1.ExternalAdmissionHook

/// ExternalAdmissionHook describes an external admission webhook and the resources and operations it applies to.
#[derive(Debug, Default)]
pub struct ExternalAdmissionHook {
    /// ClientConfig defines how to communicate with the hook. Required
    pub client_config: ::v1_8::api::admissionregistration::v1alpha1::AdmissionHookClientConfig,

    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Ignore.
    pub failure_policy: Option<String>,

    /// The name of the external admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where "imagepolicy" is the name of the webhook, and kubernetes.io is the name of the organization. Required.
    pub name: String,

    /// Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule.
    pub rules: Option<Vec<::v1_8::api::admissionregistration::v1alpha1::RuleWithOperations>>,
}

impl<'de> ::serde::Deserialize<'de> for ExternalAdmissionHook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_client_config,
            Key_failure_policy,
            Key_name,
            Key_rules,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "clientConfig" => Field::Key_client_config,
                            "failurePolicy" => Field::Key_failure_policy,
                            "name" => Field::Key_name,
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ExternalAdmissionHook;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ExternalAdmissionHook")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_client_config: Option<::v1_8::api::admissionregistration::v1alpha1::AdmissionHookClientConfig> = None;
                let mut value_failure_policy: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_rules: Option<Vec<::v1_8::api::admissionregistration::v1alpha1::RuleWithOperations>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_client_config => value_client_config = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_failure_policy => value_failure_policy = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_rules => value_rules = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ExternalAdmissionHook {
                    client_config: value_client_config.ok_or_else(|| ::serde::de::Error::missing_field("clientConfig"))?,
                    failure_policy: value_failure_policy,
                    name: value_name.ok_or_else(|| ::serde::de::Error::missing_field("name"))?,
                    rules: value_rules,
                })
            }
        }

        deserializer.deserialize_struct(
            "ExternalAdmissionHook",
            &[
                "clientConfig",
                "failurePolicy",
                "name",
                "rules",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ExternalAdmissionHook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ExternalAdmissionHook",
            0 +
            1 +
            (if self.failure_policy.is_some() { 1 } else { 0 }) +
            1 +
            (if self.rules.is_some() { 1 } else { 0 }),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "clientConfig", &self.client_config)?;
        if let Some(value) = &self.failure_policy {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "failurePolicy", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.rules {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
