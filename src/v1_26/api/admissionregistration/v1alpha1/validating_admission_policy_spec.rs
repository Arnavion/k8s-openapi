// Generated from definition io.k8s.api.admissionregistration.v1alpha1.ValidatingAdmissionPolicySpec

/// ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidatingAdmissionPolicySpec {
    /// FailurePolicy defines how to handle failures for the admission policy. Failures can occur from invalid or mis-configured policy definitions or bindings. A policy is invalid if spec.paramKind refers to a non-existent Kind. A binding is invalid if spec.paramRef.name refers to a non-existent resource. Allowed values are Ignore or Fail. Defaults to Fail.
    pub failure_policy: Option<String>,

    /// MatchConstraints specifies what resources this policy is designed to validate. The AdmissionPolicy cares about a request if it matches _all_ Constraints. However, in order to prevent clusters from being put into an unstable state that cannot be recovered from via the API ValidatingAdmissionPolicy cannot match ValidatingAdmissionPolicy and ValidatingAdmissionPolicyBinding. Required.
    pub match_constraints: Option<crate::api::admissionregistration::v1alpha1::MatchResources>,

    /// ParamKind specifies the kind of resources used to parameterize this policy. If absent, there are no parameters for this policy and the param CEL variable will not be provided to validation expressions. If ParamKind refers to a non-existent kind, this policy definition is mis-configured and the FailurePolicy is applied. If paramKind is specified but paramRef is unset in ValidatingAdmissionPolicyBinding, the params variable will be null.
    pub param_kind: Option<crate::api::admissionregistration::v1alpha1::ParamKind>,

    /// Validations contain CEL expressions which is used to apply the validation. A minimum of one validation is required for a policy definition. Required.
    pub validations: Vec<crate::api::admissionregistration::v1alpha1::Validation>,
}

impl crate::DeepMerge for ValidatingAdmissionPolicySpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.failure_policy, other.failure_policy);
        crate::DeepMerge::merge_from(&mut self.match_constraints, other.match_constraints);
        crate::DeepMerge::merge_from(&mut self.param_kind, other.param_kind);
        crate::merge_strategies::list::atomic(&mut self.validations, other.validations);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ValidatingAdmissionPolicySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_failure_policy,
            Key_match_constraints,
            Key_param_kind,
            Key_validations,
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
                            "failurePolicy" => Field::Key_failure_policy,
                            "matchConstraints" => Field::Key_match_constraints,
                            "paramKind" => Field::Key_param_kind,
                            "validations" => Field::Key_validations,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ValidatingAdmissionPolicySpec;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ValidatingAdmissionPolicySpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_failure_policy: Option<String> = None;
                let mut value_match_constraints: Option<crate::api::admissionregistration::v1alpha1::MatchResources> = None;
                let mut value_param_kind: Option<crate::api::admissionregistration::v1alpha1::ParamKind> = None;
                let mut value_validations: Option<Vec<crate::api::admissionregistration::v1alpha1::Validation>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_failure_policy => value_failure_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_constraints => value_match_constraints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_param_kind => value_param_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_validations => value_validations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ValidatingAdmissionPolicySpec {
                    failure_policy: value_failure_policy,
                    match_constraints: value_match_constraints,
                    param_kind: value_param_kind,
                    validations: value_validations.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ValidatingAdmissionPolicySpec",
            &[
                "failurePolicy",
                "matchConstraints",
                "paramKind",
                "validations",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ValidatingAdmissionPolicySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ValidatingAdmissionPolicySpec",
            1 +
            self.failure_policy.as_ref().map_or(0, |_| 1) +
            self.match_constraints.as_ref().map_or(0, |_| 1) +
            self.param_kind.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.failure_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failurePolicy", value)?;
        }
        if let Some(value) = &self.match_constraints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchConstraints", value)?;
        }
        if let Some(value) = &self.param_kind {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "paramKind", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "validations", &self.validations)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ValidatingAdmissionPolicySpec {
    fn schema_name() -> String {
        "io.k8s.api.admissionregistration.v1alpha1.ValidatingAdmissionPolicySpec".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "failurePolicy".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("FailurePolicy defines how to handle failures for the admission policy. Failures can occur from invalid or mis-configured policy definitions or bindings. A policy is invalid if spec.paramKind refers to a non-existent Kind. A binding is invalid if spec.paramRef.name refers to a non-existent resource. Allowed values are Ignore or Fail. Defaults to Fail.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "matchConstraints".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1alpha1::MatchResources>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("MatchConstraints specifies what resources this policy is designed to validate. The AdmissionPolicy cares about a request if it matches _all_ Constraints. However, in order to prevent clusters from being put into an unstable state that cannot be recovered from via the API ValidatingAdmissionPolicy cannot match ValidatingAdmissionPolicy and ValidatingAdmissionPolicyBinding. Required.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "paramKind".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1alpha1::ParamKind>().into_object();
                            schema_obj.metadata = Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("ParamKind specifies the kind of resources used to parameterize this policy. If absent, there are no parameters for this policy and the param CEL variable will not be provided to validation expressions. If ParamKind refers to a non-existent kind, this policy definition is mis-configured and the FailurePolicy is applied. If paramKind is specified but paramRef is unset in ValidatingAdmissionPolicyBinding, the params variable will be null.".to_owned()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "validations".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("Validations contain CEL expressions which is used to apply the validation. A minimum of one validation is required for a policy definition. Required.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(__gen.subschema_for::<crate::api::admissionregistration::v1alpha1::Validation>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "validations".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
