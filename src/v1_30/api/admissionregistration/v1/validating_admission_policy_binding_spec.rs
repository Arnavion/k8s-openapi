// Generated from definition io.k8s.api.admissionregistration.v1.ValidatingAdmissionPolicyBindingSpec

/// ValidatingAdmissionPolicyBindingSpec is the specification of the ValidatingAdmissionPolicyBinding.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidatingAdmissionPolicyBindingSpec {
    /// MatchResources declares what resources match this binding and will be validated by it. Note that this is intersected with the policy's matchConstraints, so only requests that are matched by the policy can be selected by this. If this is unset, all resources matched by the policy are validated by this binding When resourceRules is unset, it does not constrain resource matching. If a resource is matched by the other fields of this object, it will be validated. Note that this is differs from ValidatingAdmissionPolicy matchConstraints, where resourceRules are required.
    pub match_resources: Option<crate::api::admissionregistration::v1::MatchResources>,

    /// paramRef specifies the parameter resource used to configure the admission control policy. It should point to a resource of the type specified in ParamKind of the bound ValidatingAdmissionPolicy. If the policy specifies a ParamKind and the resource referred to by ParamRef does not exist, this binding is considered mis-configured and the FailurePolicy of the ValidatingAdmissionPolicy applied. If the policy does not specify a ParamKind then this field is ignored, and the rules are evaluated without a param.
    pub param_ref: Option<crate::api::admissionregistration::v1::ParamRef>,

    /// PolicyName references a ValidatingAdmissionPolicy name which the ValidatingAdmissionPolicyBinding binds to. If the referenced resource does not exist, this binding is considered invalid and will be ignored Required.
    pub policy_name: Option<std::string::String>,

    /// validationActions declares how Validations of the referenced ValidatingAdmissionPolicy are enforced. If a validation evaluates to false it is always enforced according to these actions.
    ///
    /// Failures defined by the ValidatingAdmissionPolicy's FailurePolicy are enforced according to these actions only if the FailurePolicy is set to Fail, otherwise the failures are ignored. This includes compilation errors, runtime errors and misconfigurations of the policy.
    ///
    /// validationActions is declared as a set of action values. Order does not matter. validationActions may not contain duplicates of the same action.
    ///
    /// The supported actions values are:
    ///
    /// "Deny" specifies that a validation failure results in a denied request.
    ///
    /// "Warn" specifies that a validation failure is reported to the request client in HTTP Warning headers, with a warning code of 299. Warnings can be sent both for allowed or denied admission responses.
    ///
    /// "Audit" specifies that a validation failure is included in the published audit event for the request. The audit event will contain a `validation.policy.admission.k8s.io/validation_failure` audit annotation with a value containing the details of the validation failures, formatted as a JSON list of objects, each with the following fields: - message: The validation failure message string - policy: The resource name of the ValidatingAdmissionPolicy - binding: The resource name of the ValidatingAdmissionPolicyBinding - expressionIndex: The index of the failed validations in the ValidatingAdmissionPolicy - validationActions: The enforcement actions enacted for the validation failure Example audit annotation: `"validation.policy.admission.k8s.io/validation_failure": "\[{"message": "Invalid value", {"policy": "policy.example.com", {"binding": "policybinding.example.com", {"expressionIndex": "1", {"validationActions": \["Audit"\]}\]"`
    ///
    /// Clients should expect to handle additional values by ignoring any values not recognized.
    ///
    /// "Deny" and "Warn" may not be used together since this combination needlessly duplicates the validation failure both in the API response body and the HTTP warning headers.
    ///
    /// Required.
    pub validation_actions: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for ValidatingAdmissionPolicyBindingSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.match_resources, other.match_resources);
        crate::DeepMerge::merge_from(&mut self.param_ref, other.param_ref);
        crate::DeepMerge::merge_from(&mut self.policy_name, other.policy_name);
        crate::merge_strategies::list::set(&mut self.validation_actions, other.validation_actions);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ValidatingAdmissionPolicyBindingSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_match_resources,
            Key_param_ref,
            Key_policy_name,
            Key_validation_actions,
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
                            "validationActions" => Field::Key_validation_actions,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ValidatingAdmissionPolicyBindingSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_match_resources: Option<crate::api::admissionregistration::v1::MatchResources> = None;
                let mut value_param_ref: Option<crate::api::admissionregistration::v1::ParamRef> = None;
                let mut value_policy_name: Option<std::string::String> = None;
                let mut value_validation_actions: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_match_resources => value_match_resources = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_param_ref => value_param_ref = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_policy_name => value_policy_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_validation_actions => value_validation_actions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ValidatingAdmissionPolicyBindingSpec {
                    match_resources: value_match_resources,
                    param_ref: value_param_ref,
                    policy_name: value_policy_name,
                    validation_actions: value_validation_actions,
                })
            }
        }

        deserializer.deserialize_struct(
            "ValidatingAdmissionPolicyBindingSpec",
            &[
                "matchResources",
                "paramRef",
                "policyName",
                "validationActions",
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
            self.policy_name.as_ref().map_or(0, |_| 1) +
            self.validation_actions.as_ref().map_or(0, |_| 1),
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
        if let Some(value) = &self.validation_actions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "validationActions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ValidatingAdmissionPolicyBindingSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.admissionregistration.v1.ValidatingAdmissionPolicyBindingSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ValidatingAdmissionPolicyBindingSpec is the specification of the ValidatingAdmissionPolicyBinding.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "matchResources".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1::MatchResources>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("MatchResources declares what resources match this binding and will be validated by it. Note that this is intersected with the policy's matchConstraints, so only requests that are matched by the policy can be selected by this. If this is unset, all resources matched by the policy are validated by this binding When resourceRules is unset, it does not constrain resource matching. If a resource is matched by the other fields of this object, it will be validated. Note that this is differs from ValidatingAdmissionPolicy matchConstraints, where resourceRules are required.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "paramRef".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1::ParamRef>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("paramRef specifies the parameter resource used to configure the admission control policy. It should point to a resource of the type specified in ParamKind of the bound ValidatingAdmissionPolicy. If the policy specifies a ParamKind and the resource referred to by ParamRef does not exist, this binding is considered mis-configured and the FailurePolicy of the ValidatingAdmissionPolicy applied. If the policy does not specify a ParamKind then this field is ignored, and the rules are evaluated without a param.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "policyName".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("PolicyName references a ValidatingAdmissionPolicy name which the ValidatingAdmissionPolicyBinding binds to. If the referenced resource does not exist, this binding is considered invalid and will be ignored Required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "validationActions".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("validationActions declares how Validations of the referenced ValidatingAdmissionPolicy are enforced. If a validation evaluates to false it is always enforced according to these actions.\n\nFailures defined by the ValidatingAdmissionPolicy's FailurePolicy are enforced according to these actions only if the FailurePolicy is set to Fail, otherwise the failures are ignored. This includes compilation errors, runtime errors and misconfigurations of the policy.\n\nvalidationActions is declared as a set of action values. Order does not matter. validationActions may not contain duplicates of the same action.\n\nThe supported actions values are:\n\n\"Deny\" specifies that a validation failure results in a denied request.\n\n\"Warn\" specifies that a validation failure is reported to the request client in HTTP Warning headers, with a warning code of 299. Warnings can be sent both for allowed or denied admission responses.\n\n\"Audit\" specifies that a validation failure is included in the published audit event for the request. The audit event will contain a `validation.policy.admission.k8s.io/validation_failure` audit annotation with a value containing the details of the validation failures, formatted as a JSON list of objects, each with the following fields: - message: The validation failure message string - policy: The resource name of the ValidatingAdmissionPolicy - binding: The resource name of the ValidatingAdmissionPolicyBinding - expressionIndex: The index of the failed validations in the ValidatingAdmissionPolicy - validationActions: The enforcement actions enacted for the validation failure Example audit annotation: `\"validation.policy.admission.k8s.io/validation_failure\": \"[{\"message\": \"Invalid value\", {\"policy\": \"policy.example.com\", {\"binding\": \"policybinding.example.com\", {\"expressionIndex\": \"1\", {\"validationActions\": [\"Audit\"]}]\"`\n\nClients should expect to handle additional values by ignoring any values not recognized.\n\n\"Deny\" and \"Warn\" may not be used together since this combination needlessly duplicates the validation failure both in the API response body and the HTTP warning headers.\n\nRequired.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(
                                    crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                                        instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                                        ..Default::default()
                                    })
                                ))),
                                ..Default::default()
                            })),
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
