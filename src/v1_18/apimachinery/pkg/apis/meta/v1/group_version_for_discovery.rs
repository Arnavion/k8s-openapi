// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery

/// GroupVersion contains the "group/version" and "version" string of a version. It is made a struct to keep extensibility.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GroupVersionForDiscovery {
    /// groupVersion specifies the API group and version in the form "group/version"
    pub group_version: String,

    /// version specifies the version in the form of "version". This is to save the clients the trouble of splitting the GroupVersion.
    pub version: String,
}

impl<'de> serde::Deserialize<'de> for GroupVersionForDiscovery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_group_version,
            Key_version,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GroupVersionForDiscovery;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("GroupVersionForDiscovery")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_group_version: Option<String> = None;
                let mut value_version: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_group_version => value_group_version = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_version => value_version = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(GroupVersionForDiscovery {
                    group_version: value_group_version.ok_or_else(|| serde::de::Error::missing_field("groupVersion"))?,
                    version: value_version.ok_or_else(|| serde::de::Error::missing_field("version"))?,
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

impl serde::Serialize for GroupVersionForDiscovery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "GroupVersionForDiscovery",
            2,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "groupVersion", &self.group_version)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "version", &self.version)?;
        serde::ser::SerializeStruct::end(state)
    }
}
