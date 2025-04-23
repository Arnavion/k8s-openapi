// Generated from definition io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.ValidationRule

/// ValidationRule describes a validation rule written in the CEL expression language.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidationRule {
    /// fieldPath represents the field path returned when the validation fails. It must be a relative JSON path (i.e. with array notation) scoped to the location of this x-kubernetes-validations extension in the schema and refer to an existing field. e.g. when validation checks if a specific attribute `foo` under a map `testMap`, the fieldPath could be set to `.testMap.foo` If the validation checks two lists must have unique attributes, the fieldPath could be set to either of the list: e.g. `.testList` It does not support list numeric index. It supports child operation to refer to an existing field currently. Refer to \[JSONPath support in Kubernetes\](https://kubernetes.io/docs/reference/kubectl/jsonpath/) for more info. Numeric index of array is not supported. For field name which contains special characters, use `\['specialName'\]` to refer the field name. e.g. for attribute `foo.34$` appears in a list `testList`, the fieldPath could be set to `.testList\['foo.34$'\]`
    pub field_path: Option<std::string::String>,

    /// Message represents the message displayed when validation fails. The message is required if the Rule contains line breaks. The message must not contain line breaks. If unset, the message is "failed rule: {Rule}". e.g. "must be a URL with the host matching spec.host"
    pub message: Option<std::string::String>,

    /// MessageExpression declares a CEL expression that evaluates to the validation failure message that is returned when this rule fails. Since messageExpression is used as a failure message, it must evaluate to a string. If both message and messageExpression are present on a rule, then messageExpression will be used if validation fails. If messageExpression results in a runtime error, the runtime error is logged, and the validation failure message is produced as if the messageExpression field were unset. If messageExpression evaluates to an empty string, a string with only spaces, or a string that contains line breaks, then the validation failure message will also be produced as if the messageExpression field were unset, and the fact that messageExpression produced an empty string/string with only spaces/string with line breaks will be logged. messageExpression has access to all the same variables as the rule; the only difference is the return type. Example: "x must be less than max ("+string(self.max)+")"
    pub message_expression: Option<std::string::String>,

    /// optionalOldSelf is used to opt a transition rule into evaluation even when the object is first created, or if the old object is missing the value.
    ///
    /// When enabled `oldSelf` will be a CEL optional whose value will be `None` if there is no old value, or when the object is initially created.
    ///
    /// You may check for presence of oldSelf using `oldSelf.hasValue()` and unwrap it after checking using `oldSelf.value()`. Check the CEL documentation for Optional types for more information: https://pkg.go.dev/github.com/google/cel-go/cel#OptionalTypes
    ///
    /// May not be set unless `oldSelf` is used in `rule`.
    pub optional_old_self: Option<bool>,

    /// reason provides a machine-readable validation failure reason that is returned to the caller when a request fails this validation rule. The HTTP status code returned to the caller will match the reason of the reason of the first failed validation rule. The currently supported reasons are: "FieldValueInvalid", "FieldValueForbidden", "FieldValueRequired", "FieldValueDuplicate". If not set, default to use "FieldValueInvalid". All future added reasons must be accepted by clients when reading this value and unknown reasons should be treated as FieldValueInvalid.
    pub reason: Option<std::string::String>,

    /// Rule represents the expression which will be evaluated by CEL. ref: https://github.com/google/cel-spec The Rule is scoped to the location of the x-kubernetes-validations extension in the schema. The `self` variable in the CEL expression is bound to the scoped value. Example: - Rule scoped to the root of a resource with a status subresource: {"rule": "self.status.actual \<= self.spec.maxDesired"}
    ///
    /// If the Rule is scoped to an object with properties, the accessible properties of the object are field selectable via `self.field` and field presence can be checked via `has(self.field)`. Null valued fields are treated as absent fields in CEL expressions. If the Rule is scoped to an object with additionalProperties (i.e. a map) the value of the map are accessible via `self\[mapKey\]`, map containment can be checked via `mapKey in self` and all entries of the map are accessible via CEL macros and functions such as `self.all(...)`. If the Rule is scoped to an array, the elements of the array are accessible via `self\[i\]` and also by macros and functions. If the Rule is scoped to a scalar, `self` is bound to the scalar value. Examples: - Rule scoped to a map of objects: {"rule": "self.components\['Widget'\].priority \< 10"} - Rule scoped to a list of integers: {"rule": "self.values.all(value, value \>= 0 && value \< 100)"} - Rule scoped to a string value: {"rule": "self.startsWith('kube')"}
    ///
    /// The `apiVersion`, `kind`, `metadata.name` and `metadata.generateName` are always accessible from the root of the object and from any x-kubernetes-embedded-resource annotated objects. No other metadata properties are accessible.
    ///
    /// Unknown data preserved in custom resources via x-kubernetes-preserve-unknown-fields is not accessible in CEL expressions. This includes: - Unknown field values that are preserved by object schemas with x-kubernetes-preserve-unknown-fields. - Object properties where the property schema is of an "unknown type". An "unknown type" is recursively defined as:
    ///   - A schema with no type and x-kubernetes-preserve-unknown-fields set to true
    ///   - An array where the items schema is of an "unknown type"
    ///   - An object where the additionalProperties schema is of an "unknown type"
    ///
    /// Only property names of the form `\[a-zA-Z_.-/\]\[a-zA-Z0-9_.-/\]*` are accessible. Accessible property names are escaped according to the following rules when accessed in the expression: - '__' escapes to '__underscores__' - '.' escapes to '__dot__' - '-' escapes to '__dash__' - '/' escapes to '__slash__' - Property names that exactly match a CEL RESERVED keyword escape to '__{keyword}__'. The keywords are:
    ///       "true", "false", "null", "in", "as", "break", "const", "continue", "else", "for", "function", "if",
    ///       "import", "let", "loop", "package", "namespace", "return".
    /// Examples:
    ///   - Rule accessing a property named "namespace": {"rule": "self.__namespace__ \> 0"}
    ///   - Rule accessing a property named "x-prop": {"rule": "self.x__dash__prop \> 0"}
    ///   - Rule accessing a property named "redact__d": {"rule": "self.redact__underscores__d \> 0"}
    ///
    /// Equality on arrays with x-kubernetes-list-type of 'set' or 'map' ignores element order, i.e. \[1, 2\] == \[2, 1\]. Concatenation on arrays with x-kubernetes-list-type use the semantics of the list type:
    ///   - 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and
    ///     non-intersecting elements in `Y` are appended, retaining their partial order.
    ///   - 'map': `X + Y` performs a merge where the array positions of all keys in `X` are preserved but the values
    ///     are overwritten by values in `Y` when the key sets of `X` and `Y` intersect. Elements in `Y` with
    ///     non-intersecting keys are appended, retaining their partial order.
    ///
    /// If `rule` makes use of the `oldSelf` variable it is implicitly a `transition rule`.
    ///
    /// By default, the `oldSelf` variable is the same type as `self`. When `optionalOldSelf` is true, the `oldSelf` variable is a CEL optional
    ///  variable whose value() is the same type as `self`.
    /// See the documentation for the `optionalOldSelf` field for details.
    ///
    /// Transition rules by default are applied only on UPDATE requests and are skipped if an old value could not be found. You can opt a transition rule into unconditional evaluation by setting `optionalOldSelf` to true.
    pub rule: std::string::String,
}

impl crate::DeepMerge for ValidationRule {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.field_path, other.field_path);
        crate::DeepMerge::merge_from(&mut self.message, other.message);
        crate::DeepMerge::merge_from(&mut self.message_expression, other.message_expression);
        crate::DeepMerge::merge_from(&mut self.optional_old_self, other.optional_old_self);
        crate::DeepMerge::merge_from(&mut self.reason, other.reason);
        crate::DeepMerge::merge_from(&mut self.rule, other.rule);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ValidationRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_field_path,
            Key_message,
            Key_message_expression,
            Key_optional_old_self,
            Key_reason,
            Key_rule,
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
                            "fieldPath" => Field::Key_field_path,
                            "message" => Field::Key_message,
                            "messageExpression" => Field::Key_message_expression,
                            "optionalOldSelf" => Field::Key_optional_old_self,
                            "reason" => Field::Key_reason,
                            "rule" => Field::Key_rule,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ValidationRule;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ValidationRule")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_field_path: Option<std::string::String> = None;
                let mut value_message: Option<std::string::String> = None;
                let mut value_message_expression: Option<std::string::String> = None;
                let mut value_optional_old_self: Option<bool> = None;
                let mut value_reason: Option<std::string::String> = None;
                let mut value_rule: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_field_path => value_field_path = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message_expression => value_message_expression = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_optional_old_self => value_optional_old_self = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rule => value_rule = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ValidationRule {
                    field_path: value_field_path,
                    message: value_message,
                    message_expression: value_message_expression,
                    optional_old_self: value_optional_old_self,
                    reason: value_reason,
                    rule: value_rule.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "ValidationRule",
            &[
                "fieldPath",
                "message",
                "messageExpression",
                "optionalOldSelf",
                "reason",
                "rule",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ValidationRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ValidationRule",
            1 +
            self.field_path.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.message_expression.as_ref().map_or(0, |_| 1) +
            self.optional_old_self.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.field_path {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "fieldPath", value)?;
        }
        if let Some(value) = &self.message {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.message_expression {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "messageExpression", value)?;
        }
        if let Some(value) = &self.optional_old_self {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "optionalOldSelf", value)?;
        }
        if let Some(value) = &self.reason {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rule", &self.rule)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ValidationRule {
    fn schema_name() -> std::string::String {
        "io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1.ValidationRule".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ValidationRule describes a validation rule written in the CEL expression language.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "fieldPath".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("fieldPath represents the field path returned when the validation fails. It must be a relative JSON path (i.e. with array notation) scoped to the location of this x-kubernetes-validations extension in the schema and refer to an existing field. e.g. when validation checks if a specific attribute `foo` under a map `testMap`, the fieldPath could be set to `.testMap.foo` If the validation checks two lists must have unique attributes, the fieldPath could be set to either of the list: e.g. `.testList` It does not support list numeric index. It supports child operation to refer to an existing field currently. Refer to [JSONPath support in Kubernetes](https://kubernetes.io/docs/reference/kubectl/jsonpath/) for more info. Numeric index of array is not supported. For field name which contains special characters, use `['specialName']` to refer the field name. e.g. for attribute `foo.34$` appears in a list `testList`, the fieldPath could be set to `.testList['foo.34$']`".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "message".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Message represents the message displayed when validation fails. The message is required if the Rule contains line breaks. The message must not contain line breaks. If unset, the message is \"failed rule: {Rule}\". e.g. \"must be a URL with the host matching spec.host\"".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "messageExpression".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("MessageExpression declares a CEL expression that evaluates to the validation failure message that is returned when this rule fails. Since messageExpression is used as a failure message, it must evaluate to a string. If both message and messageExpression are present on a rule, then messageExpression will be used if validation fails. If messageExpression results in a runtime error, the runtime error is logged, and the validation failure message is produced as if the messageExpression field were unset. If messageExpression evaluates to an empty string, a string with only spaces, or a string that contains line breaks, then the validation failure message will also be produced as if the messageExpression field were unset, and the fact that messageExpression produced an empty string/string with only spaces/string with line breaks will be logged. messageExpression has access to all the same variables as the rule; the only difference is the return type. Example: \"x must be less than max (\"+string(self.max)+\")\"".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "optionalOldSelf".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("optionalOldSelf is used to opt a transition rule into evaluation even when the object is first created, or if the old object is missing the value.\n\nWhen enabled `oldSelf` will be a CEL optional whose value will be `None` if there is no old value, or when the object is initially created.\n\nYou may check for presence of oldSelf using `oldSelf.hasValue()` and unwrap it after checking using `oldSelf.value()`. Check the CEL documentation for Optional types for more information: https://pkg.go.dev/github.com/google/cel-go/cel#OptionalTypes\n\nMay not be set unless `oldSelf` is used in `rule`.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "reason".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("reason provides a machine-readable validation failure reason that is returned to the caller when a request fails this validation rule. The HTTP status code returned to the caller will match the reason of the reason of the first failed validation rule. The currently supported reasons are: \"FieldValueInvalid\", \"FieldValueForbidden\", \"FieldValueRequired\", \"FieldValueDuplicate\". If not set, default to use \"FieldValueInvalid\". All future added reasons must be accepted by clients when reading this value and unknown reasons should be treated as FieldValueInvalid.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "rule".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("Rule represents the expression which will be evaluated by CEL. ref: https://github.com/google/cel-spec The Rule is scoped to the location of the x-kubernetes-validations extension in the schema. The `self` variable in the CEL expression is bound to the scoped value. Example: - Rule scoped to the root of a resource with a status subresource: {\"rule\": \"self.status.actual <= self.spec.maxDesired\"}\n\nIf the Rule is scoped to an object with properties, the accessible properties of the object are field selectable via `self.field` and field presence can be checked via `has(self.field)`. Null valued fields are treated as absent fields in CEL expressions. If the Rule is scoped to an object with additionalProperties (i.e. a map) the value of the map are accessible via `self[mapKey]`, map containment can be checked via `mapKey in self` and all entries of the map are accessible via CEL macros and functions such as `self.all(...)`. If the Rule is scoped to an array, the elements of the array are accessible via `self[i]` and also by macros and functions. If the Rule is scoped to a scalar, `self` is bound to the scalar value. Examples: - Rule scoped to a map of objects: {\"rule\": \"self.components['Widget'].priority < 10\"} - Rule scoped to a list of integers: {\"rule\": \"self.values.all(value, value >= 0 && value < 100)\"} - Rule scoped to a string value: {\"rule\": \"self.startsWith('kube')\"}\n\nThe `apiVersion`, `kind`, `metadata.name` and `metadata.generateName` are always accessible from the root of the object and from any x-kubernetes-embedded-resource annotated objects. No other metadata properties are accessible.\n\nUnknown data preserved in custom resources via x-kubernetes-preserve-unknown-fields is not accessible in CEL expressions. This includes: - Unknown field values that are preserved by object schemas with x-kubernetes-preserve-unknown-fields. - Object properties where the property schema is of an \"unknown type\". An \"unknown type\" is recursively defined as:\n  - A schema with no type and x-kubernetes-preserve-unknown-fields set to true\n  - An array where the items schema is of an \"unknown type\"\n  - An object where the additionalProperties schema is of an \"unknown type\"\n\nOnly property names of the form `[a-zA-Z_.-/][a-zA-Z0-9_.-/]*` are accessible. Accessible property names are escaped according to the following rules when accessed in the expression: - '__' escapes to '__underscores__' - '.' escapes to '__dot__' - '-' escapes to '__dash__' - '/' escapes to '__slash__' - Property names that exactly match a CEL RESERVED keyword escape to '__{keyword}__'. The keywords are:\n\t  \"true\", \"false\", \"null\", \"in\", \"as\", \"break\", \"const\", \"continue\", \"else\", \"for\", \"function\", \"if\",\n\t  \"import\", \"let\", \"loop\", \"package\", \"namespace\", \"return\".\nExamples:\n  - Rule accessing a property named \"namespace\": {\"rule\": \"self.__namespace__ > 0\"}\n  - Rule accessing a property named \"x-prop\": {\"rule\": \"self.x__dash__prop > 0\"}\n  - Rule accessing a property named \"redact__d\": {\"rule\": \"self.redact__underscores__d > 0\"}\n\nEquality on arrays with x-kubernetes-list-type of 'set' or 'map' ignores element order, i.e. [1, 2] == [2, 1]. Concatenation on arrays with x-kubernetes-list-type use the semantics of the list type:\n  - 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and\n    non-intersecting elements in `Y` are appended, retaining their partial order.\n  - 'map': `X + Y` performs a merge where the array positions of all keys in `X` are preserved but the values\n    are overwritten by values in `Y` when the key sets of `X` and `Y` intersect. Elements in `Y` with\n    non-intersecting keys are appended, retaining their partial order.\n\nIf `rule` makes use of the `oldSelf` variable it is implicitly a `transition rule`.\n\nBy default, the `oldSelf` variable is the same type as `self`. When `optionalOldSelf` is true, the `oldSelf` variable is a CEL optional\n variable whose value() is the same type as `self`.\nSee the documentation for the `optionalOldSelf` field for details.\n\nTransition rules by default are applied only on UPDATE requests and are skipped if an old value could not be found. You can opt a transition rule into unconditional evaluation by setting `optionalOldSelf` to true.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "rule".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
