// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.OwnerReference

/// OwnerReference contains enough information to let you identify an owning object. An owning object must be in the same namespace as the dependent, or be cluster-scoped, so there is no namespace field.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OwnerReference {
    /// API version of the referent.
    pub api_version: String,

    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs "delete" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.
    pub block_owner_deletion: Option<bool>,

    /// If true, this reference points to the managing controller.
    pub controller: Option<bool>,

    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: String,

    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,

    /// UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    pub uid: String,
}

impl<'de> serde::Deserialize<'de> for OwnerReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
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

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OwnerReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("OwnerReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_block_owner_deletion: Option<bool> = None;
                let mut value_controller: Option<bool> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_block_owner_deletion => value_block_owner_deletion = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_controller => value_controller = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_uid => value_uid = Some(serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(OwnerReference {
                    api_version: value_api_version.ok_or_else(|| serde::de::Error::missing_field("apiVersion"))?,
                    block_owner_deletion: value_block_owner_deletion,
                    controller: value_controller,
                    kind: value_kind.ok_or_else(|| serde::de::Error::missing_field("kind"))?,
                    name: value_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    uid: value_uid.ok_or_else(|| serde::de::Error::missing_field("uid"))?,
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

impl serde::Serialize for OwnerReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "OwnerReference",
            4 +
            self.block_owner_deletion.as_ref().map_or(0, |_| 1) +
            self.controller.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", &self.api_version)?;
        if let Some(value) = &self.block_owner_deletion {
            serde::ser::SerializeStruct::serialize_field(&mut state, "blockOwnerDeletion", value)?;
        }
        if let Some(value) = &self.controller {
            serde::ser::SerializeStruct::serialize_field(&mut state, "controller", value)?;
        }
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "uid", &self.uid)?;
        serde::ser::SerializeStruct::end(state)
    }
}
