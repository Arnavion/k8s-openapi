// Generated from definition io.k8s.api.flowcontrol.v1beta1.PolicyRulesWithSubjects

/// PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver. The test considers the subject making the request, the verb being requested, and the resource to be acted upon. This PolicyRulesWithSubjects matches a request if and only if both (a) at least one member of subjects matches the request and (b) at least one member of resourceRules or nonResourceRules matches the request.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PolicyRulesWithSubjects {
    /// `nonResourceRules` is a list of NonResourcePolicyRules that identify matching requests according to their verb and the target non-resource URL.
    pub non_resource_rules: Option<Vec<crate::api::flowcontrol::v1beta1::NonResourcePolicyRule>>,

    /// `resourceRules` is a slice of ResourcePolicyRules that identify matching requests according to their verb and the target resource. At least one of `resourceRules` and `nonResourceRules` has to be non-empty.
    pub resource_rules: Option<Vec<crate::api::flowcontrol::v1beta1::ResourcePolicyRule>>,

    /// subjects is the list of normal user, serviceaccount, or group that this rule cares about. There must be at least one member in this slice. A slice that includes both the system:authenticated and system:unauthenticated user groups matches every request. Required.
    pub subjects: Vec<crate::api::flowcontrol::v1beta1::Subject>,
}

impl<'de> serde::Deserialize<'de> for PolicyRulesWithSubjects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_non_resource_rules,
            Key_resource_rules,
            Key_subjects,
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
                            "nonResourceRules" => Field::Key_non_resource_rules,
                            "resourceRules" => Field::Key_resource_rules,
                            "subjects" => Field::Key_subjects,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PolicyRulesWithSubjects;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("PolicyRulesWithSubjects")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_non_resource_rules: Option<Vec<crate::api::flowcontrol::v1beta1::NonResourcePolicyRule>> = None;
                let mut value_resource_rules: Option<Vec<crate::api::flowcontrol::v1beta1::ResourcePolicyRule>> = None;
                let mut value_subjects: Option<Vec<crate::api::flowcontrol::v1beta1::Subject>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_non_resource_rules => value_non_resource_rules = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_rules => value_resource_rules = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_subjects => value_subjects = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PolicyRulesWithSubjects {
                    non_resource_rules: value_non_resource_rules,
                    resource_rules: value_resource_rules,
                    subjects: value_subjects.ok_or_else(|| serde::de::Error::missing_field("subjects"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PolicyRulesWithSubjects",
            &[
                "nonResourceRules",
                "resourceRules",
                "subjects",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for PolicyRulesWithSubjects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PolicyRulesWithSubjects",
            1 +
            self.non_resource_rules.as_ref().map_or(0, |_| 1) +
            self.resource_rules.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.non_resource_rules {
            serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceRules", value)?;
        }
        if let Some(value) = &self.resource_rules {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resourceRules", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "subjects", &self.subjects)?;
        serde::ser::SerializeStruct::end(state)
    }
}
