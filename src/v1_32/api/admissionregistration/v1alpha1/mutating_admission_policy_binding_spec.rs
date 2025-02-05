// Generated from definition io.k8s.api.admissionregistration.v1alpha1.MutatingAdmissionPolicyBindingSpec

/// MutatingAdmissionPolicyBindingSpec is the specification of the MutatingAdmissionPolicyBinding.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MutatingAdmissionPolicyBindingSpec {
    /// matchResources limits what resources match this binding and may be mutated by it. Note that if matchResources matches a resource, the resource must also match a policy's matchConstraints and matchConditions before the resource may be mutated. When matchResources is unset, it does not constrain resource matching, and only the policy's matchConstraints and matchConditions must match for the resource to be mutated. Additionally, matchResources.resourceRules are optional and do not constraint matching when unset. Note that this is differs from MutatingAdmissionPolicy matchConstraints, where resourceRules are required. The CREATE, UPDATE and CONNECT operations are allowed.  The DELETE operation may not be matched. '*' matches CREATE, UPDATE and CONNECT.
    pub match_resources: Option<crate::api::admissionregistration::v1alpha1::MatchResources>,

    /// paramRef specifies the parameter resource used to configure the admission control policy. It should point to a resource of the type specified in spec.ParamKind of the bound MutatingAdmissionPolicy. If the policy specifies a ParamKind and the resource referred to by ParamRef does not exist, this binding is considered mis-configured and the FailurePolicy of the MutatingAdmissionPolicy applied. If the policy does not specify a ParamKind then this field is ignored, and the rules are evaluated without a param.
    pub param_ref: Option<crate::api::admissionregistration::v1alpha1::ParamRef>,

    /// policyName references a MutatingAdmissionPolicy name which the MutatingAdmissionPolicyBinding binds to. If the referenced resource does not exist, this binding is considered invalid and will be ignored Required.
    pub policy_name: Option<std::string::String>,
}

impl crate::DeepMerge for MutatingAdmissionPolicyBindingSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.match_resources, other.match_resources);
        crate::DeepMerge::merge_from(&mut self.param_ref, other.param_ref);
        crate::DeepMerge::merge_from(&mut self.policy_name, other.policy_name);
    }
}

impl<'de> crate::serde::Deserialize<'de> for MutatingAdmissionPolicyBindingSpec {
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

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
            type Value = MutatingAdmissionPolicyBindingSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("MutatingAdmissionPolicyBindingSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_match_resources: Option<crate::api::admissionregistration::v1alpha1::MatchResources> = None;
                let mut value_param_ref: Option<crate::api::admissionregistration::v1alpha1::ParamRef> = None;
                let mut value_policy_name: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_resources => value_match_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_param_ref => value_param_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_policy_name => value_policy_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(MutatingAdmissionPolicyBindingSpec {
                    match_resources: value_match_resources,
                    param_ref: value_param_ref,
                    policy_name: value_policy_name,
                })
            }
        }

        deserializer.deserialize_struct(
            "MutatingAdmissionPolicyBindingSpec",
            &[
                "matchResources",
                "paramRef",
                "policyName",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for MutatingAdmissionPolicyBindingSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "MutatingAdmissionPolicyBindingSpec",
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
impl crate::schemars::JsonSchema for MutatingAdmissionPolicyBindingSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.admissionregistration.v1alpha1.MutatingAdmissionPolicyBindingSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("MutatingAdmissionPolicyBindingSpec is the specification of the MutatingAdmissionPolicyBinding.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "matchResources".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1alpha1::MatchResources>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("matchResources limits what resources match this binding and may be mutated by it. Note that if matchResources matches a resource, the resource must also match a policy's matchConstraints and matchConditions before the resource may be mutated. When matchResources is unset, it does not constrain resource matching, and only the policy's matchConstraints and matchConditions must match for the resource to be mutated. Additionally, matchResources.resourceRules are optional and do not constraint matching when unset. Note that this is differs from MutatingAdmissionPolicy matchConstraints, where resourceRules are required. The CREATE, UPDATE and CONNECT operations are allowed.  The DELETE operation may not be matched. '*' matches CREATE, UPDATE and CONNECT.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "paramRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1alpha1::ParamRef>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("paramRef specifies the parameter resource used to configure the admission control policy. It should point to a resource of the type specified in spec.ParamKind of the bound MutatingAdmissionPolicy. If the policy specifies a ParamKind and the resource referred to by ParamRef does not exist, this binding is considered mis-configured and the FailurePolicy of the MutatingAdmissionPolicy applied. If the policy does not specify a ParamKind then this field is ignored, and the rules are evaluated without a param.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "policyName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("policyName references a MutatingAdmissionPolicy name which the MutatingAdmissionPolicyBinding binds to. If the referenced resource does not exist, this binding is considered invalid and will be ignored Required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
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
