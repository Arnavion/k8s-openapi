// Generated from definition io.k8s.api.admissionregistration.v1alpha1.JSONPatch

/// JSONPatch defines a JSON Patch.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JSONPatch {
    /// expression will be evaluated by CEL to create a \[JSON patch\](https://jsonpatch.com/). ref: https://github.com/google/cel-spec
    ///
    /// expression must return an array of JSONPatch values.
    ///
    /// For example, this CEL expression returns a JSON patch to conditionally modify a value:
    ///
    ///   \[
    ///         JSONPatch{op: "test", path: "/spec/example", value: "Red"},
    ///         JSONPatch{op: "replace", path: "/spec/example", value: "Green"}
    ///       \]
    ///
    /// To define an object for the patch value, use Object types. For example:
    ///
    ///   \[
    ///         JSONPatch{
    ///           op: "add",
    ///           path: "/spec/selector",
    ///           value: Object.spec.selector{matchLabels: {"environment": "test"}}
    ///         }
    ///       \]
    ///
    /// To use strings containing '/' and '~' as JSONPatch path keys, use "jsonpatch.escapeKey". For example:
    ///
    ///   \[
    ///         JSONPatch{
    ///           op: "add",
    ///           path: "/metadata/labels/" + jsonpatch.escapeKey("example.com/environment"),
    ///           value: "test"
    ///         },
    ///       \]
    ///
    /// CEL expressions have access to the types needed to create JSON patches and objects:
    ///
    /// - 'JSONPatch' - CEL type of JSON Patch operations. JSONPatch has the fields 'op', 'from', 'path' and 'value'.
    ///   See \[JSON patch\](https://jsonpatch.com/) for more details. The 'value' field may be set to any of: string,
    ///   integer, array, map or object.  If set, the 'path' and 'from' fields must be set to a
    ///   \[JSON pointer\](https://datatracker.ietf.org/doc/html/rfc6901/) string, where the 'jsonpatch.escapeKey()' CEL
    ///   function may be used to escape path keys containing '/' and '~'.
    /// - 'Object' - CEL type of the resource object. - 'Object.\<fieldName\>' - CEL type of object field (such as 'Object.spec') - 'Object.\<fieldName1\>.\<fieldName2\>...\<fieldNameN\>` - CEL type of nested field (such as 'Object.spec.containers')
    ///
    /// CEL expressions have access to the contents of the API request, organized into CEL variables as well as some other useful variables:
    ///
    /// - 'object' - The object from the incoming request. The value is null for DELETE requests. - 'oldObject' - The existing object. The value is null for CREATE requests. - 'request' - Attributes of the API request(\[ref\](/pkg/apis/admission/types.go#AdmissionRequest)). - 'params' - Parameter resource referred to by the policy binding being evaluated. Only populated if the policy has a ParamKind. - 'namespaceObject' - The namespace object that the incoming object belongs to. The value is null for cluster-scoped resources. - 'variables' - Map of composited variables, from its name to its lazily evaluated value.
    ///   For example, a variable named 'foo' can be accessed as 'variables.foo'.
    /// - 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.
    ///   See https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz
    /// - 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the
    ///   request resource.
    ///
    /// CEL expressions have access to \[Kubernetes CEL function libraries\](https://kubernetes.io/docs/reference/using-api/cel/#cel-options-language-features-and-libraries) as well as:
    ///
    /// - 'jsonpatch.escapeKey' - Performs JSONPatch key escaping. '~' and  '/' are escaped as '~0' and `~1' respectively).
    ///
    /// Only property names of the form `\[a-zA-Z_.-/\]\[a-zA-Z0-9_.-/\]*` are accessible. Required.
    pub expression: Option<std::string::String>,
}

impl crate::DeepMerge for JSONPatch {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.expression, other.expression);
    }
}

impl<'de> crate::serde::Deserialize<'de> for JSONPatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_expression,
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
                            "expression" => Field::Key_expression,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = JSONPatch;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("JSONPatch")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_expression: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_expression => value_expression = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(JSONPatch {
                    expression: value_expression,
                })
            }
        }

        deserializer.deserialize_struct(
            "JSONPatch",
            &[
                "expression",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for JSONPatch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "JSONPatch",
            self.expression.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.expression {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "expression", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for JSONPatch {
    fn schema_name() -> std::string::String {
        "io.k8s.api.admissionregistration.v1alpha1.JSONPatch".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("JSONPatch defines a JSON Patch.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "expression".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("expression will be evaluated by CEL to create a [JSON patch](https://jsonpatch.com/). ref: https://github.com/google/cel-spec\n\nexpression must return an array of JSONPatch values.\n\nFor example, this CEL expression returns a JSON patch to conditionally modify a value:\n\n\t  [\n\t    JSONPatch{op: \"test\", path: \"/spec/example\", value: \"Red\"},\n\t    JSONPatch{op: \"replace\", path: \"/spec/example\", value: \"Green\"}\n\t  ]\n\nTo define an object for the patch value, use Object types. For example:\n\n\t  [\n\t    JSONPatch{\n\t      op: \"add\",\n\t      path: \"/spec/selector\",\n\t      value: Object.spec.selector{matchLabels: {\"environment\": \"test\"}}\n\t    }\n\t  ]\n\nTo use strings containing '/' and '~' as JSONPatch path keys, use \"jsonpatch.escapeKey\". For example:\n\n\t  [\n\t    JSONPatch{\n\t      op: \"add\",\n\t      path: \"/metadata/labels/\" + jsonpatch.escapeKey(\"example.com/environment\"),\n\t      value: \"test\"\n\t    },\n\t  ]\n\nCEL expressions have access to the types needed to create JSON patches and objects:\n\n- 'JSONPatch' - CEL type of JSON Patch operations. JSONPatch has the fields 'op', 'from', 'path' and 'value'.\n  See [JSON patch](https://jsonpatch.com/) for more details. The 'value' field may be set to any of: string,\n  integer, array, map or object.  If set, the 'path' and 'from' fields must be set to a\n  [JSON pointer](https://datatracker.ietf.org/doc/html/rfc6901/) string, where the 'jsonpatch.escapeKey()' CEL\n  function may be used to escape path keys containing '/' and '~'.\n- 'Object' - CEL type of the resource object. - 'Object.<fieldName>' - CEL type of object field (such as 'Object.spec') - 'Object.<fieldName1>.<fieldName2>...<fieldNameN>` - CEL type of nested field (such as 'Object.spec.containers')\n\nCEL expressions have access to the contents of the API request, organized into CEL variables as well as some other useful variables:\n\n- 'object' - The object from the incoming request. The value is null for DELETE requests. - 'oldObject' - The existing object. The value is null for CREATE requests. - 'request' - Attributes of the API request([ref](/pkg/apis/admission/types.go#AdmissionRequest)). - 'params' - Parameter resource referred to by the policy binding being evaluated. Only populated if the policy has a ParamKind. - 'namespaceObject' - The namespace object that the incoming object belongs to. The value is null for cluster-scoped resources. - 'variables' - Map of composited variables, from its name to its lazily evaluated value.\n  For example, a variable named 'foo' can be accessed as 'variables.foo'.\n- 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.\n  See https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz\n- 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the\n  request resource.\n\nCEL expressions have access to [Kubernetes CEL function libraries](https://kubernetes.io/docs/reference/using-api/cel/#cel-options-language-features-and-libraries) as well as:\n\n- 'jsonpatch.escapeKey' - Performs JSONPatch key escaping. '~' and  '/' are escaped as '~0' and `~1' respectively).\n\nOnly property names of the form `[a-zA-Z_.-/][a-zA-Z0-9_.-/]*` are accessible. Required.".into()),
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
