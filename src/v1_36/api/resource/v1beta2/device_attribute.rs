// Generated from definition io.k8s.api.resource.v1beta2.DeviceAttribute

/// DeviceAttribute must have exactly one field set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeviceAttribute {
    /// BoolValue is a true/false value.
    pub bool: Option<bool>,

    /// BoolValues is a non-empty list of true/false values.
    pub bools: Option<std::vec::Vec<bool>>,

    /// IntValue is a number.
    pub int: Option<i64>,

    /// IntValues is a non-empty list of numbers.
    ///
    /// This is an alpha field and requires enabling the DRAListTypeAttributes feature gate.
    pub ints: Option<std::vec::Vec<i64>>,

    /// StringValue is a string. Must not be longer than 64 characters.
    pub string: Option<std::string::String>,

    /// StringValues is a non-empty list of strings. Each string must not be longer than 64 characters.
    ///
    /// This is an alpha field and requires enabling the DRAListTypeAttributes feature gate.
    pub strings: Option<std::vec::Vec<std::string::String>>,

    /// VersionValue is a semantic version according to semver.org spec 2.0.0. Must not be longer than 64 characters.
    pub version: Option<std::string::String>,

    /// VersionValues is a non-empty list of semantic versions according to semver.org spec 2.0.0. Each version string must not be longer than 64 characters.
    ///
    /// This is an alpha field and requires enabling the DRAListTypeAttributes feature gate.
    pub versions: Option<std::vec::Vec<std::string::String>>,
}

impl crate::DeepMerge for DeviceAttribute {
    fn merge_from(&mut self, other: Self) {
        crate::DeepMerge::merge_from(&mut self.bool, other.bool);
        crate::merge_strategies::list::atomic(&mut self.bools, other.bools);
        crate::DeepMerge::merge_from(&mut self.int, other.int);
        crate::merge_strategies::list::atomic(&mut self.ints, other.ints);
        crate::DeepMerge::merge_from(&mut self.string, other.string);
        crate::merge_strategies::list::atomic(&mut self.strings, other.strings);
        crate::DeepMerge::merge_from(&mut self.version, other.version);
        crate::merge_strategies::list::atomic(&mut self.versions, other.versions);
    }
}

impl<'de> crate::serde::Deserialize<'de> for DeviceAttribute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_bool,
            Key_bools,
            Key_int,
            Key_ints,
            Key_string,
            Key_strings,
            Key_version,
            Key_versions,
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
                            "bools" => Field::Key_bools,
                            "int" => Field::Key_int,
                            "ints" => Field::Key_ints,
                            "string" => Field::Key_string,
                            "strings" => Field::Key_strings,
                            "version" => Field::Key_version,
                            "versions" => Field::Key_versions,
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
                let mut value_bools: Option<std::vec::Vec<bool>> = None;
                let mut value_int: Option<i64> = None;
                let mut value_ints: Option<std::vec::Vec<i64>> = None;
                let mut value_string: Option<std::string::String> = None;
                let mut value_strings: Option<std::vec::Vec<std::string::String>> = None;
                let mut value_version: Option<std::string::String> = None;
                let mut value_versions: Option<std::vec::Vec<std::string::String>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_bool => value_bool = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_bools => value_bools = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_int => value_int = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_ints => value_ints = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_string => value_string = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_strings => value_strings = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_version => value_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_versions => value_versions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(DeviceAttribute {
                    bool: value_bool,
                    bools: value_bools,
                    int: value_int,
                    ints: value_ints,
                    string: value_string,
                    strings: value_strings,
                    version: value_version,
                    versions: value_versions,
                })
            }
        }

        deserializer.deserialize_struct(
            "DeviceAttribute",
            &[
                "bool",
                "bools",
                "int",
                "ints",
                "string",
                "strings",
                "version",
                "versions",
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
            self.bools.as_ref().map_or(0, |_| 1) +
            self.int.as_ref().map_or(0, |_| 1) +
            self.ints.as_ref().map_or(0, |_| 1) +
            self.string.as_ref().map_or(0, |_| 1) +
            self.strings.as_ref().map_or(0, |_| 1) +
            self.version.as_ref().map_or(0, |_| 1) +
            self.versions.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.bool {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bool", value)?;
        }
        if let Some(value) = &self.bools {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "bools", value)?;
        }
        if let Some(value) = &self.int {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "int", value)?;
        }
        if let Some(value) = &self.ints {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "ints", value)?;
        }
        if let Some(value) = &self.string {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "string", value)?;
        }
        if let Some(value) = &self.strings {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "strings", value)?;
        }
        if let Some(value) = &self.version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "version", value)?;
        }
        if let Some(value) = &self.versions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "versions", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for DeviceAttribute {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.api.resource.v1beta2.DeviceAttribute".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "DeviceAttribute must have exactly one field set.",
            "type": "object",
            "properties": {
                "bool": {
                    "description": "BoolValue is a true/false value.",
                    "type": "boolean",
                },
                "bools": {
                    "description": "BoolValues is a non-empty list of true/false values.",
                    "type": "array",
                    "items": {
                        "type": "boolean",
                    },
                },
                "int": {
                    "description": "IntValue is a number.",
                    "type": "integer",
                    "format": "int64",
                },
                "ints": {
                    "description": "IntValues is a non-empty list of numbers.\n\nThis is an alpha field and requires enabling the DRAListTypeAttributes feature gate.",
                    "type": "array",
                    "items": {
                        "type": "integer",
                        "format": "int64",
                    },
                },
                "string": {
                    "description": "StringValue is a string. Must not be longer than 64 characters.",
                    "type": "string",
                },
                "strings": {
                    "description": "StringValues is a non-empty list of strings. Each string must not be longer than 64 characters.\n\nThis is an alpha field and requires enabling the DRAListTypeAttributes feature gate.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
                "version": {
                    "description": "VersionValue is a semantic version according to semver.org spec 2.0.0. Must not be longer than 64 characters.",
                    "type": "string",
                },
                "versions": {
                    "description": "VersionValues is a non-empty list of semantic versions according to semver.org spec 2.0.0. Each version string must not be longer than 64 characters.\n\nThis is an alpha field and requires enabling the DRAListTypeAttributes feature gate.",
                    "type": "array",
                    "items": {
                        "type": "string",
                    },
                },
            },
        })
    }
}
