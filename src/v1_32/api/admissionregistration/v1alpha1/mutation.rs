// Generated from definition io.k8s.api.admissionregistration.v1alpha1.Mutation

/// Mutation specifies the CEL expression which is used to apply the Mutation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mutation {
    /// applyConfiguration defines the desired configuration values of an object. The configuration is applied to the admission object using \[structured merge diff\](https://github.com/kubernetes-sigs/structured-merge-diff). A CEL expression is used to create apply configuration.
    pub apply_configuration: Option<crate::api::admissionregistration::v1alpha1::ApplyConfiguration>,

    /// jsonPatch defines a \[JSON patch\](https://jsonpatch.com/) operation to perform a mutation to the object. A CEL expression is used to create the JSON patch.
    pub json_patch: Option<crate::api::admissionregistration::v1alpha1::JSONPatch>,

    /// patchType indicates the patch strategy used. Allowed values are "ApplyConfiguration" and "JSONPatch". Required.
    pub patch_type: std::string::String,
}

impl crate::DeepMerge for Mutation {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.apply_configuration, other.apply_configuration);
        crate::DeepMerge::merge_from(&mut self.json_patch, other.json_patch);
        crate::DeepMerge::merge_from(&mut self.patch_type, other.patch_type);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Mutation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_apply_configuration,
            Key_json_patch,
            Key_patch_type,
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
                            "applyConfiguration" => Field::Key_apply_configuration,
                            "jsonPatch" => Field::Key_json_patch,
                            "patchType" => Field::Key_patch_type,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Mutation;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Mutation")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_apply_configuration: Option<crate::api::admissionregistration::v1alpha1::ApplyConfiguration> = None;
                let mut value_json_patch: Option<crate::api::admissionregistration::v1alpha1::JSONPatch> = None;
                let mut value_patch_type: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_apply_configuration => value_apply_configuration = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_json_patch => value_json_patch = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_patch_type => value_patch_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Mutation {
                    apply_configuration: value_apply_configuration,
                    json_patch: value_json_patch,
                    patch_type: value_patch_type.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "Mutation",
            &[
                "applyConfiguration",
                "jsonPatch",
                "patchType",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Mutation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Mutation",
            1 +
            self.apply_configuration.as_ref().map_or(0, |_| 1) +
            self.json_patch.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.apply_configuration {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "applyConfiguration", value)?;
        }
        if let Some(value) = &self.json_patch {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "jsonPatch", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "patchType", &self.patch_type)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Mutation {
    fn schema_name() -> std::string::String {
        "io.k8s.api.admissionregistration.v1alpha1.Mutation".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("Mutation specifies the CEL expression which is used to apply the Mutation.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "applyConfiguration".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1alpha1::ApplyConfiguration>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("applyConfiguration defines the desired configuration values of an object. The configuration is applied to the admission object using [structured merge diff](https://github.com/kubernetes-sigs/structured-merge-diff). A CEL expression is used to create apply configuration.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "jsonPatch".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::admissionregistration::v1alpha1::JSONPatch>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("jsonPatch defines a [JSON patch](https://jsonpatch.com/) operation to perform a mutation to the object. A CEL expression is used to create the JSON patch.".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "patchType".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("patchType indicates the patch strategy used. Allowed values are \"ApplyConfiguration\" and \"JSONPatch\". Required.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "patchType".into(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
