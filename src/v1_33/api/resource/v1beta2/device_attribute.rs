// Generated from definition io.k8s.api.resource.v1beta2.DeviceAttribute

/// DeviceAttribute must have exactly one field set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceAttribute {
    /// BoolValue is a true/false value.
    pub bool: Option<bool>,

    /// IntValue is a number.
    pub int: Option<i64>,

    /// StringValue is a string. Must not be longer than 64 characters.
    pub string: Option<std::string::String>,

    /// VersionValue is a semantic version according to semver.org spec 2.0.0. Must not be longer than 64 characters.
    pub version: Option<std::string::String>,
}

impl crate::DeepMerge for DeviceAttribute {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.bool, other.bool);
        crate::DeepMerge::merge_from(&mut self.int, other.int);
        crate::DeepMerge::merge_from(&mut self.string, other.string);
        crate::DeepMerge::merge_from(&mut self.version, other.version);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bool,
            Key_int,
            Key_string,
            Key_version,
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
                            "bool" => Field::Key_bool,
                            "int" => Field::Key_int,
                            "string" => Field::Key_string,
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
            type Value = DeviceAttribute;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("DeviceAttribute")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_bool: Option<bool> = None;
                let mut value_int: Option<i64> = None;
                let mut value_string: Option<std::string::String> = None;
                let mut value_version: Option<std::string::String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bool => value_bool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_int => value_int = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_string => value_string = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceAttribute {
                    bool: value_bool,
                    int: value_int,
                    string: value_string,
                    version: value_version,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceAttribute",
            &[
                "bool",
                "int",
                "string",
                "version",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for DeviceAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DeviceAttribute",
            self.bool.as_ref().map_or(0, |_| 1) +
            self.int.as_ref().map_or(0, |_| 1) +
            self.string.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bool {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bool", value)?;
        }
        if let Some(value) = &self.int {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "int", value)?;
        }
        if let Some(value) = &self.string {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "string", value)?;
        }
        if let Some(value) = &self.version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceAttribute {
    fn schema_name() -> std::string::String {
        "io.k8s.api.resource.v1beta2.DeviceAttribute".into()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                description: Some("DeviceAttribute must have exactly one field set.".into()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(std::boxed::Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "bool".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("BoolValue is a true/false value.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Boolean))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "int".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("IntValue is a number.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::Integer))),
                            format: Some("int64".into()),
                            ..Default::default()
                        }),
                    ),
                    (
                        "string".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("StringValue is a string. Must not be longer than 64 characters.".into()),
                                ..Default::default()
                            })),
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(std::boxed::Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "version".into(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            metadata: Some(std::boxed::Box::new(crate::schemars::schema::Metadata {
                                description: Some("VersionValue is a semantic version according to semver.org spec 2.0.0. Must not be longer than 64 characters.".into()),
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
