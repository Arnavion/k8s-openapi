// Generated from definition io.k8s.api.admissionregistration.v1alpha1.ApplyConfiguration

/// ApplyConfiguration defines the desired configuration values of an object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApplyConfiguration {
    /// expression will be evaluated by CEL to create an apply configuration. ref: https://github.com/google/cel-spec
    ///
    /// Apply configurations are declared in CEL using object initialization. For example, this CEL expression returns an apply configuration to set a single field:
    ///
    ///   Object{
    ///       spec: Object.spec{
    ///         serviceAccountName: "example"
    ///       }
    ///     }
    ///
    /// Apply configurations may not modify atomic structs, maps or arrays due to the risk of accidental deletion of values not included in the apply configuration.
    ///
    /// CEL expressions have access to the object types needed to create apply configurations:
    ///
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
    /// The `apiVersion`, `kind`, `metadata.name` and `metadata.generateName` are always accessible from the root of the object. No other metadata properties are accessible.
    ///
    /// Only property names of the form `\[a-zA-Z_.-/\]\[a-zA-Z0-9_.-/\]*` are accessible. Required.
    pub expression: Option<std::string::String>,
}

impl crate::DeepMerge for ApplyConfiguration {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.expression, other.expression);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ApplyConfiguration {
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
            type Value = ApplyConfiguration;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ApplyConfiguration")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_expression: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_expression => value_expression = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ApplyConfiguration {
                    expression: value_expression,
                })
            }
        }

        deserializer.deserialize_struct(
            "ApplyConfiguration",
            &[
                "expression",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ApplyConfiguration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ApplyConfiguration",
            self.expression.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.expression {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "expression", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ApplyConfiguration {
    fn schema_name() -> std::string::String {
        "io.k8s.api.admissionregistration.v1alpha1.ApplyConfiguration".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ApplyConfiguration defines the desired configuration values of an object.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "expression".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("expression will be evaluated by CEL to create an apply configuration. ref: https://github.com/google/cel-spec\n\nApply configurations are declared in CEL using object initialization. For example, this CEL expression returns an apply configuration to set a single field:\n\n\tObject{\n\t  spec: Object.spec{\n\t    serviceAccountName: \"example\"\n\t  }\n\t}\n\nApply configurations may not modify atomic structs, maps or arrays due to the risk of accidental deletion of values not included in the apply configuration.\n\nCEL expressions have access to the object types needed to create apply configurations:\n\n- 'Object' - CEL type of the resource object. - 'Object.<fieldName>' - CEL type of object field (such as 'Object.spec') - 'Object.<fieldName1>.<fieldName2>...<fieldNameN>` - CEL type of nested field (such as 'Object.spec.containers')\n\nCEL expressions have access to the contents of the API request, organized into CEL variables as well as some other useful variables:\n\n- 'object' - The object from the incoming request. The value is null for DELETE requests. - 'oldObject' - The existing object. The value is null for CREATE requests. - 'request' - Attributes of the API request([ref](/pkg/apis/admission/types.go#AdmissionRequest)). - 'params' - Parameter resource referred to by the policy binding being evaluated. Only populated if the policy has a ParamKind. - 'namespaceObject' - The namespace object that the incoming object belongs to. The value is null for cluster-scoped resources. - 'variables' - Map of composited variables, from its name to its lazily evaluated value.\n  For example, a variable named 'foo' can be accessed as 'variables.foo'.\n- 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.\n  See https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz\n- 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the\n  request resource.\n\nThe `apiVersion`, `kind`, `metadata.name` and `metadata.generateName` are always accessible from the root of the object. No other metadata properties are accessible.\n\nOnly property names of the form `[a-zA-Z_.-/][a-zA-Z0-9_.-/]*` are accessible. Required.".into()),
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
