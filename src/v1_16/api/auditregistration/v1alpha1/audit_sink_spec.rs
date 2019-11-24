// Generated from definition io.k8s.api.auditregistration.v1alpha1.AuditSinkSpec

/// AuditSinkSpec holds the spec for the audit sink
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AuditSinkSpec {
    /// Policy defines the policy for selecting which events should be sent to the webhook required
    pub policy: crate::api::auditregistration::v1alpha1::Policy,

    /// Webhook to send events required
    pub webhook: crate::api::auditregistration::v1alpha1::Webhook,
}

impl<'de> serde::Deserialize<'de> for AuditSinkSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_policy,
            Key_webhook,
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
                            "policy" => Field::Key_policy,
                            "webhook" => Field::Key_webhook,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditSinkSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("AuditSinkSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_policy: Option<crate::api::auditregistration::v1alpha1::Policy> = None;
                let mut value_webhook: Option<crate::api::auditregistration::v1alpha1::Webhook> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_policy => value_policy = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_webhook => value_webhook = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(AuditSinkSpec {
                    policy: value_policy.ok_or_else(|| serde::de::Error::missing_field("policy"))?,
                    webhook: value_webhook.ok_or_else(|| serde::de::Error::missing_field("webhook"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "AuditSinkSpec",
            &[
                "policy",
                "webhook",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for AuditSinkSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "AuditSinkSpec",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "policy", &self.policy)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "webhook", &self.webhook)?;
        serde::ser::SerializeStruct::end(state)
    }
}
