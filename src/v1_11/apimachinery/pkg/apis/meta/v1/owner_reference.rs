// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.OwnerReference

/// OwnerReference contains enough information to let you identify an owning object. Currently, an owning object must be in the same namespace, so there is no namespace field.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OwnerReference {
    /// API version of the referent.
    pub api_version: String,

    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs "delete" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.
    pub block_owner_deletion: Option<bool>,

    /// If true, this reference points to the managing controller.
    pub controller: Option<bool>,

    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: String,

    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,

    /// UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    pub uid: String,
}

impl<'de> crate::serde::Deserialize<'de> for OwnerReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_block_owner_deletion,
            Key_controller,
            Key_kind,
            Key_name,
            Key_uid,
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
                            "apiVersion" => Field::Key_api_version,
                            "blockOwnerDeletion" => Field::Key_block_owner_deletion,
                            "controller" => Field::Key_controller,
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

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = OwnerReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("OwnerReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_block_owner_deletion: Option<bool> = None;
                let mut value_controller: Option<bool> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_block_owner_deletion => value_block_owner_deletion = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_controller => value_controller = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_uid => value_uid = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(OwnerReference {
                    api_version: value_api_version.ok_or_else(|| crate::serde::de::Error::missing_field("apiVersion"))?,
                    block_owner_deletion: value_block_owner_deletion,
                    controller: value_controller,
                    kind: value_kind.ok_or_else(|| crate::serde::de::Error::missing_field("kind"))?,
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    uid: value_uid.ok_or_else(|| crate::serde::de::Error::missing_field("uid"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "OwnerReference",
            &[
                "apiVersion",
                "blockOwnerDeletion",
                "controller",
                "kind",
                "name",
                "uid",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for OwnerReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "OwnerReference",
            4 +
            self.block_owner_deletion.as_ref().map_or(0, |_| 1) +
            self.controller.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", &self.api_version)?;
        if let Some(value) = &self.block_owner_deletion {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "blockOwnerDeletion", value)?;
        }
        if let Some(value) = &self.controller {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "controller", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", &self.uid)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for OwnerReference {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "OwnerReference contains enough information to let you identify an owning object. Currently, an owning object must be in the same namespace, so there is no namespace field.",
          "properties": {
            "apiVersion": {
              "description": "API version of the referent.",
              "type": "string"
            },
            "blockOwnerDeletion": {
              "description": "If true, AND if the owner has the \"foregroundDeletion\" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs \"delete\" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.",
              "type": "boolean"
            },
            "controller": {
              "description": "If true, this reference points to the managing controller.",
              "type": "boolean"
            },
            "kind": {
              "description": "Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds",
              "type": "string"
            },
            "name": {
              "description": "Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names",
              "type": "string"
            },
            "uid": {
              "description": "UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids",
              "type": "string"
            }
          },
          "required": [
            "apiVersion",
            "kind",
            "name",
            "uid"
          ],
          "type": "object"
        })
    }
}
