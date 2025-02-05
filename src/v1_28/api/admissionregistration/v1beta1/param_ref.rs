// Generated from definition io.k8s.api.admissionregistration.v1beta1.ParamRef

/// ParamRef describes how to locate the params to be used as input to expressions of rules applied by a policy binding.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParamRef {
    /// name is the name of the resource being referenced.
    ///
    /// One of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is set, the other must be unset.
    ///
    /// A single parameter used for all admission requests can be configured by setting the `name` field, leaving `selector` blank, and setting namespace if `paramKind` is namespace-scoped.
    pub name: Option<std::string::String>,

    /// namespace is the namespace of the referenced resource. Allows limiting the search for params to a specific namespace. Applies to both `name` and `selector` fields.
    ///
    /// A per-namespace parameter may be used by specifying a namespace-scoped `paramKind` in the policy and leaving this field empty.
    ///
    /// - If `paramKind` is cluster-scoped, this field MUST be unset. Setting this field results in a configuration error.
    ///
    /// - If `paramKind` is namespace-scoped, the namespace of the object being evaluated for admission will be used when this field is left unset. Take care that if this is left empty the binding must not match any cluster-scoped resources, which will result in an error.
    pub namespace: Option<std::string::String>,

    /// `parameterNotFoundAction` controls the behavior of the binding when the resource exists, and name or selector is valid, but there are no parameters matched by the binding. If the value is set to `Allow`, then no matched parameters will be treated as successful validation by the binding. If set to `Deny`, then no matched parameters will be subject to the `failurePolicy` of the policy.
    ///
    /// Allowed values are `Allow` or `Deny`
    ///
    /// Required
    pub parameter_not_found_action: Option<std::string::String>,

    /// selector can be used to match multiple param objects based on their labels. Supply selector: {} to match all resources of the ParamKind.
    ///
    /// If multiple params are found, they are all evaluated with the policy expressions and the results are ANDed together.
    ///
    /// One of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is set, the other must be unset.
    pub selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

impl crate::DeepMerge for ParamRef {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.name, other.name);
        crate::DeepMerge::merge_from(&mut self.namespace, other.namespace);
        crate::DeepMerge::merge_from(&mut self.parameter_not_found_action, other.parameter_not_found_action);
        crate::DeepMerge::merge_from(&mut self.selector, other.selector);
    }
}

impl<'de> crate::serde::Deserialize<'de> for ParamRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_name,
            Key_namespace,
            Key_parameter_not_found_action,
            Key_selector,
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
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            "parameterNotFoundAction" => Field::Key_parameter_not_found_action,
                            "selector" => Field::Key_selector,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ParamRef;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("ParamRef")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_name: Option<std::string::String> = None;
                let mut value_namespace: Option<std::string::String> = None;
                let mut value_parameter_not_found_action: Option<std::string::String> = None;
                let mut value_selector: Option<crate::apimachinery::pkg::apis::meta::v1::LabelSelector> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_name => value_name = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace => value_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameter_not_found_action => value_parameter_not_found_action = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_selector => value_selector = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ParamRef {
                    name: value_name,
                    namespace: value_namespace,
                    parameter_not_found_action: value_parameter_not_found_action,
                    selector: value_selector,
                })
            }
        }

        deserializer.deserialize_struct(
            "ParamRef",
            &[
                "name",
                "namespace",
                "parameterNotFoundAction",
                "selector",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ParamRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ParamRef",
            self.name.as_ref().map_or(0, |_| 1) +
            self.namespace.as_ref().map_or(0, |_| 1) +
            self.parameter_not_found_action.as_ref().map_or(0, |_| 1) +
            self.selector.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.name {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        if let Some(value) = &self.parameter_not_found_action {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "parameterNotFoundAction", value)?;
        }
        if let Some(value) = &self.selector {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selector", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for ParamRef {
    fn schema_name() -> std::string::String {
        "io.k8s.api.admissionregistration.v1beta1.ParamRef".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("ParamRef describes how to locate the params to be used as input to expressions of rules applied by a policy binding.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "name".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("name is the name of the resource being referenced.\n\nOne of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is set, the other must be unset.\n\nA single parameter used for all admission requests can be configured by setting the `name` field, leaving `selector` blank, and setting namespace if `paramKind` is namespace-scoped.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "namespace".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("namespace is the namespace of the referenced resource. Allows limiting the search for params to a specific namespace. Applies to both `name` and `selector` fields.\n\nA per-namespace parameter may be used by specifying a namespace-scoped `paramKind` in the policy and leaving this field empty.\n\n- If `paramKind` is cluster-scoped, this field MUST be unset. Setting this field results in a configuration error.\n\n- If `paramKind` is namespace-scoped, the namespace of the object being evaluated for admission will be used when this field is left unset. Take care that if this is left empty the binding must not match any cluster-scoped resources, which will result in an error.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "parameterNotFoundAction".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("`parameterNotFoundAction` controls the behavior of the binding when the resource exists, and name or selector is valid, but there are no parameters matched by the binding. If the value is set to `Allow`, then no matched parameters will be treated as successful validation by the binding. If set to `Deny`, then no matched parameters will be subject to the `failurePolicy` of the policy.\n\nAllowed values are `Allow` or `Deny`\n\nRequired".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "selector".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::LabelSelector>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("selector can be used to match multiple param objects based on their labels. Supply selector: {} to match all resources of the ParamKind.\n\nIf multiple params are found, they are all evaluated with the policy expressions and the results are ANDed together.\n\nOne of `name` or `selector` must be set, but `name` and `selector` are mutually exclusive properties. If one is set, the other must be unset.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
