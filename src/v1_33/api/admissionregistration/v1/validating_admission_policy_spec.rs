// Generated from definition io.k8s.api.admissionregistration.v1.ValidatingAdmissionPolicySpec

/// ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidatingAdmissionPolicySpec {
    /// auditAnnotations contains CEL expressions which are used to produce audit annotations for the audit event of the API request. validations and auditAnnotations may not both be empty; a least one of validations or auditAnnotations is required.
    pub audit_annotations: Option<std::vec::Vec<crate::api::admissionregistration::v1::AuditAnnotation>>,

    /// failurePolicy defines how to handle failures for the admission policy. Failures can occur from CEL expression parse errors, type check errors, runtime errors and invalid or mis-configured policy definitions or bindings.
    ///
    /// A policy is invalid if spec.paramKind refers to a non-existent Kind. A binding is invalid if spec.paramRef.name refers to a non-existent resource.
    ///
    /// failurePolicy does not define how validations that evaluate to false are handled.
    ///
    /// When failurePolicy is set to Fail, ValidatingAdmissionPolicyBinding validationActions define how failures are enforced.
    ///
    /// Allowed values are Ignore or Fail. Defaults to Fail.
    pub failure_policy: Option<std::string::String>,

    /// MatchConditions is a list of conditions that must be met for a request to be validated. Match conditions filter requests that have already been matched by the rules, namespaceSelector, and objectSelector. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions allowed.
    ///
    /// If a parameter object is provided, it can be accessed via the `params` handle in the same manner as validation expressions.
    ///
    /// The exact matching logic is (in order):
    ///   1. If ANY matchCondition evaluates to FALSE, the policy is skipped.
    ///   2. If ALL matchConditions evaluate to TRUE, the policy is evaluated.
    ///   3. If any matchCondition evaluates to an error (but none are FALSE):
    ///      - If failurePolicy=Fail, reject the request
    ///      - If failurePolicy=Ignore, the policy is skipped
    pub match_conditions: Option<std::vec::Vec<crate::api::admissionregistration::v1::MatchCondition>>,

    /// MatchConstraints specifies what resources this policy is designed to validate. The AdmissionPolicy cares about a request if it matches _all_ Constraints. However, in order to prevent clusters from being put into an unstable state that cannot be recovered from via the API ValidatingAdmissionPolicy cannot match ValidatingAdmissionPolicy and ValidatingAdmissionPolicyBinding. Required.
    pub match_constraints: Option<crate::api::admissionregistration::v1::MatchResources>,

    /// ParamKind specifies the kind of resources used to parameterize this policy. If absent, there are no parameters for this policy and the param CEL variable will not be provided to validation expressions. If ParamKind refers to a non-existent kind, this policy definition is mis-configured and the FailurePolicy is applied. If paramKind is specified but paramRef is unset in ValidatingAdmissionPolicyBinding, the params variable will be null.
    pub param_kind: Option<crate::api::admissionregistration::v1::ParamKind>,

    /// Validations contain CEL expressions which is used to apply the validation. Validations and AuditAnnotations may not both be empty; a minimum of one Validations or AuditAnnotations is required.
    pub validations: Option<std::vec::Vec<crate::api::admissionregistration::v1::Validation>>,

    /// Variables contain definitions of variables that can be used in composition of other expressions. Each variable is defined as a named CEL expression. The variables defined here will be available under `variables` in other expressions of the policy except MatchConditions because MatchConditions are evaluated before the rest of the policy.
    ///
    /// The expression of a variable can refer to other variables defined earlier in the list but not those after. Thus, Variables must be sorted by the order of first appearance and acyclic.
    pub variables: Option<std::vec::Vec<crate::api::admissionregistration::v1::Variable>>,
}

impl crate::DeepMerge for ValidatingAdmissionPolicySpec {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.audit_annotations, other.audit_annotations);
        crate::DeepMerge::merge_from(&mut self.failure_policy, other.failure_policy);
        crate::merge_strategies::list::map(
            &mut self.match_conditions,
            other.match_conditions,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
        crate::DeepMerge::merge_from(&mut self.match_constraints, other.match_constraints);
        crate::DeepMerge::merge_from(&mut self.param_kind, other.param_kind);
        crate::merge_strategies::list::atomic(&mut self.validations, other.validations);
        crate::merge_strategies::list::map(
            &mut self.variables,
            other.variables,
            &[|lhs, rhs| lhs.name == rhs.name],
            |current_item, other_item| {
                crate::DeepMerge::merge_from(current_item, other_item);
            },
        );
    }
}

impl<'de> crate::serde::Deserialize<'de> for ValidatingAdmissionPolicySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_audit_annotations,
            Key_failure_policy,
            Key_match_conditions,
            Key_match_constraints,
            Key_param_kind,
            Key_validations,
            Key_variables,
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
                            "auditAnnotations" => Field::Key_audit_annotations,
                            "failurePolicy" => Field::Key_failure_policy,
                            "matchConditions" => Field::Key_match_conditions,
                            "matchConstraints" => Field::Key_match_constraints,
                            "paramKind" => Field::Key_param_kind,
                            "validations" => Field::Key_validations,
                            "variables" => Field::Key_variables,
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

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ValidatingAdmissionPolicySpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_audit_annotations: Option<std::vec::Vec<crate::api::admissionregistration::v1::AuditAnnotation>> = None;
                let mut value_failure_policy: Option<std::string::String> = None;
                let mut value_match_conditions: Option<std::vec::Vec<crate::api::admissionregistration::v1::MatchCondition>> = None;
                let mut value_match_constraints: Option<crate::api::admissionregistration::v1::MatchResources> = None;
                let mut value_param_kind: Option<crate::api::admissionregistration::v1::ParamKind> = None;
                let mut value_validations: Option<std::vec::Vec<crate::api::admissionregistration::v1::Validation>> = None;
                let mut value_variables: Option<std::vec::Vec<crate::api::admissionregistration::v1::Variable>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_audit_annotations => value_audit_annotations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_failure_policy => value_failure_policy = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_conditions => value_match_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_match_constraints => value_match_constraints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_param_kind => value_param_kind = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_validations => value_validations = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_variables => value_variables = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ValidatingAdmissionPolicySpec {
                    audit_annotations: value_audit_annotations,
                    failure_policy: value_failure_policy,
                    match_conditions: value_match_conditions,
                    match_constraints: value_match_constraints,
                    param_kind: value_param_kind,
                    validations: value_validations,
                    variables: value_variables,
                })
            }
        }

        deserializer.deserialize_struct(
            "ValidatingAdmissionPolicySpec",
            &[
                "auditAnnotations",
                "failurePolicy",
                "matchConditions",
                "matchConstraints",
                "paramKind",
                "validations",
                "variables",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ValidatingAdmissionPolicySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ValidatingAdmissionPolicySpec",
            self.audit_annotations.as_ref().map_or(0, |_| 1) +
            self.failure_policy.as_ref().map_or(0, |_| 1) +
            self.match_conditions.as_ref().map_or(0, |_| 1) +
            self.match_constraints.as_ref().map_or(0, |_| 1) +
            self.param_kind.as_ref().map_or(0, |_| 1) +
            self.validations.as_ref().map_or(0, |_| 1) +
            self.variables.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.audit_annotations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "auditAnnotations", value)?;
        }
        if let Some(value) = &self.failure_policy {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "failurePolicy", value)?;
        }
        if let Some(value) = &self.match_conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchConditions", value)?;
        }
        if let Some(value) = &self.match_constraints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "matchConstraints", value)?;
        }
        if let Some(value) = &self.param_kind {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "paramKind", value)?;
        }
        if let Some(value) = &self.validations {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "validations", value)?;
        }
        if let Some(value) = &self.variables {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "variables", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ValidatingAdmissionPolicySpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.admissionregistration.v1.ValidatingAdmissionPolicySpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "auditAnnotations".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("auditAnnotations contains CEL expressions which are used to produce audit annotations for the audit event of the API request. validations and auditAnnotations may not both be empty; a least one of validations or auditAnnotations is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::admissionregistration::v1::AuditAnnotation>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "failurePolicy".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("failurePolicy defines how to handle failures for the admission policy. Failures can occur from CEL expression parse errors, type check errors, runtime errors and invalid or mis-configured policy definitions or bindings.\n\nA policy is invalid if spec.paramKind refers to a non-existent Kind. A binding is invalid if spec.paramRef.name refers to a non-existent resource.\n\nfailurePolicy does not define how validations that evaluate to false are handled.\n\nWhen failurePolicy is set to Fail, ValidatingAdmissionPolicyBinding validationActions define how failures are enforced.\n\nAllowed values are Ignore or Fail. Defaults to Fail.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "matchConditions".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("MatchConditions is a list of conditions that must be met for a request to be validated. Match conditions filter requests that have already been matched by the rules, namespaceSelector, and objectSelector. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions allowed.\n\nIf a parameter object is provided, it can be accessed via the `params` handle in the same manner as validation expressions.\n\nThe exact matching logic is (in order):\n  1. If ANY matchCondition evaluates to FALSE, the policy is skipped.\n  2. If ALL matchConditions evaluate to TRUE, the policy is evaluated.\n  3. If any matchCondition evaluates to an error (but none are FALSE):\n     - If failurePolicy=Fail, reject the request\n     - If failurePolicy=Ignore, the policy is skipped".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::admissionregistration::v1::MatchCondition>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "matchConstraints".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1::MatchResources>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("MatchConstraints specifies what resources this policy is designed to validate. The AdmissionPolicy cares about a request if it matches _all_ Constraints. However, in order to prevent clusters from being put into an unstable state that cannot be recovered from via the API ValidatingAdmissionPolicy cannot match ValidatingAdmissionPolicy and ValidatingAdmissionPolicyBinding. Required.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "paramKind".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1::ParamKind>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ParamKind specifies the kind of resources used to parameterize this policy. If absent, there are no parameters for this policy and the param CEL variable will not be provided to validation expressions. If ParamKind refers to a non-existent kind, this policy definition is mis-configured and the FailurePolicy is applied. If paramKind is specified but paramRef is unset in ValidatingAdmissionPolicyBinding, the params variable will be null.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "validations".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Validations contain CEL expressions which is used to apply the validation. Validations and AuditAnnotations may not both be empty; a minimum of one Validations or AuditAnnotations is required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::admissionregistration::v1::Validation>()))),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }),
                    ),
                    (
                        "variables".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Variables contain definitions of variables that can be used in composition of other expressions. Each variable is defined as a named CEL expression. The variables defined here will be available under `variables` in other expressions of the policy except MatchConditions because MatchConditions are evaluated before the rest of the policy.\n\nThe expression of a variable can refer to other variables defined earlier in the list but not those after. Thus, Variables must be sorted by the order of first appearance and acyclic.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Array))),
                            array: Some(std::boxed::Box::new(crate::schemars::schema::ArrayValidation {
                                items: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(__gen.subschema_for::<crate::api::admissionregistration::v1::Variable>()))),
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
