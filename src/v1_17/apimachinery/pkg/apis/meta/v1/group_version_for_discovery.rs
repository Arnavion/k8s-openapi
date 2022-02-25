// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery

/// GroupVersion contains the "group/version" and "version" string of a version. It is made a struct to keep extensibility.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GroupVersionForDiscovery {
    /// groupVersion specifies the API group and version in the form "group/version"
    pub group_version: String,

    /// version specifies the version in the form of "version". This is to save the clients the trouble of splitting the GroupVersion.
    pub version: String,
}

impl<'de> crate::serde::Deserialize<'de> for GroupVersionForDiscovery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_group_version,
            Key_version,
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
                            "groupVersion" => Field::Key_group_version,
                            "version" => Field::Key_version,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = GroupVersionForDiscovery;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GroupVersionForDiscovery")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_group_version: Option<String> = None;
                let mut value_version: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group_version => value_group_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GroupVersionForDiscovery {
                    group_version: value_group_version.unwrap_or_default(),
                    version: value_version.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            "GroupVersionForDiscovery",
            &[
                "groupVersion",
                "version",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for GroupVersionForDiscovery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GroupVersionForDiscovery",
            2,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "groupVersion", &self.group_version)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "version", &self.version)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for GroupVersionForDiscovery {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("GroupVersion contains the \"group/version\" and \"version\" string of a version. It is made a struct to keep extensibility.".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "groupVersion".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("groupVersion specifies the API group and version in the form \"group/version\"".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "version".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                                description: Some("version specifies the version in the form of \"version\". This is to save the clients the trouble of splitting the GroupVersion.".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "groupVersion".to_owned(),
                    "version".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
