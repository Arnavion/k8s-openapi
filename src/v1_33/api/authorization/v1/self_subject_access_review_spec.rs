// Generated from definition io.k8s.api.authorization.v1.SelfSubjectAccessReviewSpec

/// SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelfSubjectAccessReviewSpec {
    /// NonResourceAttributes describes information for a non-resource access request
    pub non_resource_attributes: Option<crate::api::authorization::v1::NonResourceAttributes>,

    /// ResourceAuthorizationAttributes describes information for a resource access request
    pub resource_attributes: Option<crate::api::authorization::v1::ResourceAttributes>,
}

impl crate::DeepMerge for SelfSubjectAccessReviewSpec {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.non_resource_attributes, other.non_resource_attributes);
        crate::DeepMerge::merge_from(&mut self.resource_attributes, other.resource_attributes);
    }
}

impl<'de> crate::serde::Deserialize<'de> for SelfSubjectAccessReviewSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_non_resource_attributes,
            Key_resource_attributes,
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
                            "nonResourceAttributes" => Field::Key_non_resource_attributes,
                            "resourceAttributes" => Field::Key_resource_attributes,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = SelfSubjectAccessReviewSpec;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("SelfSubjectAccessReviewSpec")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_non_resource_attributes: Option<crate::api::authorization::v1::NonResourceAttributes> = None;
                let mut value_resource_attributes: Option<crate::api::authorization::v1::ResourceAttributes> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_non_resource_attributes => value_non_resource_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_attributes => value_resource_attributes = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(SelfSubjectAccessReviewSpec {
                    non_resource_attributes: value_non_resource_attributes,
                    resource_attributes: value_resource_attributes,
                })
            }
        }

        deserializer.deserialize_struct(
            "SelfSubjectAccessReviewSpec",
            &[
                "nonResourceAttributes",
                "resourceAttributes",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for SelfSubjectAccessReviewSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "SelfSubjectAccessReviewSpec",
            self.non_resource_attributes.as_ref().map_or(0, |_| 1) +
            self.resource_attributes.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.non_resource_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "nonResourceAttributes", value)?;
        }
        if let Some(value) = &self.resource_attributes {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceAttributes", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for SelfSubjectAccessReviewSpec {
    fn schema_name() -> std::string::String {
        "io.k8s.api.authorization.v1.SelfSubjectAccessReviewSpec".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "nonResourceAttributes".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::authorization::v1::NonResourceAttributes>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("NonResourceAttributes describes information for a non-resource access request".into()),
                                ..Default::default()
                            }));
                            crate::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "resourceAttributes".into(),
                        {
                            let mut schema_obj = __gen.subschema_for::<crate::api::authorization::v1::ResourceAttributes>().into_object();
                            schema_obj.metadata = Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("ResourceAuthorizationAttributes describes information for a resource access request".into()),
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
