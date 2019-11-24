// Generated from definition io.k8s.api.authentication.v1.BoundObjectReference

/// BoundObjectReference is a reference to an object that a token is bound to.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoundObjectReference {
    /// API version of the referent.
    pub api_version: Option<String>,

    /// Kind of the referent. Valid kinds are 'Pod' and 'Secret'.
    pub kind: Option<String>,

    /// Name of the referent.
    pub name: Option<String>,

    /// UID of the referent.
    pub uid: Option<String>,
}

impl<'de> serde::Deserialize<'de> for BoundObjectReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_name,
            Key_uid,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BoundObjectReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BoundObjectReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BoundObjectReference {
                    api_version: value_api_version,
                    kind: value_kind,
                    name: value_name,
                    uid: value_uid,
                })
            }
        }

        deserializer.deserialize_struct(
            "BoundObjectReference",
            &[
                "apiVersion",
                "kind",
                "name",
                "uid",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for BoundObjectReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BoundObjectReference",
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.uid {
            serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
