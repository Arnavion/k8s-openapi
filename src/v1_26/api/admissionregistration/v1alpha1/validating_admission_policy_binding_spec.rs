// Generated from definition io.k8s.api.admissionregistration.v1alpha1.ValidatingAdmissionPolicyBindingSpec

/// ValidatingAdmissionPolicyBindingSpec is the specification of the ValidatingAdmissionPolicyBinding.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidatingAdmissionPolicyBindingSpec {
    /// MatchResources declares what resources match this binding and will be validated by it. Note that this is intersected with the policy's matchConstraints, so only requests that are matched by the policy can be selected by this. If this is unset, all resources matched by the policy are validated by this binding When resourceRules is unset, it does not constrain resource matching. If a resource is matched by the other fields of this object, it will be validated. Note that this is differs from ValidatingAdmissionPolicy matchConstraints, where resourceRules are required.
    pub match_resources: Option<crate::api::admissionregistration::v1alpha1::MatchResources>,

    /// ParamRef specifies the parameter resource used to configure the admission control policy. It should point to a resource of the type specified in ParamKind of the bound ValidatingAdmissionPolicy. If the policy specifies a ParamKind and the resource referred to by ParamRef does not exist, this binding is considered mis-configured and the FailurePolicy of the ValidatingAdmissionPolicy applied.
    pub param_ref: Option<crate::api::admissionregistration::v1alpha1::ParamRef>,

    /// PolicyName references a ValidatingAdmissionPolicy name which the ValidatingAdmissionPolicyBinding binds to. If the referenced resource does not exist, this binding is considered invalid and will be ignored Required.
    pub policy_name: Option<String>,
}

impl crate::DeepMerge for ValidatingAdmissionPolicyBindingSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.match_resources, other.match_resources);
        crate::DeepMerge::merge_from(&mut self.param_ref, other.param_ref);
        crate::DeepMerge::merge_from(&mut self.policy_name, other.policy_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ValidatingAdmissionPolicyBindingSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_resources,
            Key_param_ref,
            Key_policy_name,
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
                            "matchResources" => Field::Key_match_resources,
                            "paramRef" => Field::Key_param_ref,
                            "policyName" => Field::Key_policy_name,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ValidatingAdmissionPolicyBindingSpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ValidatingAdmissionPolicyBindingSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_match_resources: Option<crate::api::admissionregistration::v1alpha1::MatchResources> = None;
                let mut value_param_ref: Option<crate::api::admissionregistration::v1alpha1::ParamRef> = None;
                let mut value_policy_name: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_resources => value_match_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_param_ref => value_param_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_policy_name => value_policy_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ValidatingAdmissionPolicyBindingSpec {
                    match_resources: value_match_resources,
                    param_ref: value_param_ref,
                    policy_name: value_policy_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "ValidatingAdmissionPolicyBindingSpec",
            &[
                "matchResources",
                "paramRef",
                "policyName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ValidatingAdmissionPolicyBindingSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ValidatingAdmissionPolicyBindingSpec",
            self.match_resources.as_ref().map_or(0, |_| 1) +
            self.param_ref.as_ref().map_or(0, |_| 1) +
            self.policy_name.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.match_resources {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchResources", value)?;
        }
        if let Some(value) = &self.param_ref {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "paramRef", value)?;
        }
        if let Some(value) = &self.policy_name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "policyName", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ValidatingAdmissionPolicyBindingSpec {
    fn schema_name() -> String {
        "io.k8s.api.admissionregistration.v1alpha1.ValidatingAdmissionPolicyBindingSpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ValidatingAdmissionPolicyBindingSpec is the specification of the ValidatingAdmissionPolicyBinding.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "matchResources".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1alpha1::MatchResources>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MatchResources declares what resources match this binding and will be validated by it. Note that this is intersected with the policy's matchConstraints, so only requests that are matched by the policy can be selected by this. If this is unset, all resources matched by the policy are validated by this binding When resourceRules is unset, it does not constrain resource matching. If a resource is matched by the other fields of this object, it will be validated. Note that this is differs from ValidatingAdmissionPolicy matchConstraints, where resourceRules are required.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "paramRef".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1alpha1::ParamRef>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ParamRef specifies the parameter resource used to configure the admission control policy. It should point to a resource of the type specified in ParamKind of the bound ValidatingAdmissionPolicy. If the policy specifies a ParamKind and the resource referred to by ParamRef does not exist, this binding is considered mis-configured and the FailurePolicy of the ValidatingAdmissionPolicy applied.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "policyName".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("PolicyName references a ValidatingAdmissionPolicy name which the ValidatingAdmissionPolicyBinding binds to. If the referenced resource does not exist, this binding is considered invalid and will be ignored Required.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
