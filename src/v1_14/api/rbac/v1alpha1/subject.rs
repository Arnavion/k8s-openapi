// Generated from definition io.k8s.api.rbac.v1alpha1.Subject

/// Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference, or a value for non-objects such as user and group names.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Subject {
    /// APIVersion holds the API group and version of the referenced subject. Defaults to "v1" for ServiceAccount subjects. Defaults to "rbac.authorization.k8s.io/v1alpha1" for User and Group subjects.
    pub api_version: Option<String>,

    /// Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount". If the Authorizer does not recognized the kind value, the Authorizer should report an error.
    pub kind: String,

    /// Name of the object being referenced.
    pub name: String,

    /// Namespace of the referenced object.  If the object kind is non-namespace, such as "User" or "Group", and this value is not empty the Authorizer should report an error.
    pub namespace: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for Subject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_name,
            Key_namespace,
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
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Subject;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("Subject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_name => value_name = Some(crate::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_namespace => value_namespace = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Subject {
                    api_version: value_api_version,
                    kind: value_kind.ok_or_else(|| crate::serde::de::Error::missing_field("kind"))?,
                    name: value_name.ok_or_else(|| crate::serde::de::Error::missing_field("name"))?,
                    namespace: value_namespace,
                })
            }
        }

        deserializer.deserialize_struct(
            "Subject",
            &[
                "apiVersion",
                "kind",
                "name",
                "namespace",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Subject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Subject",
            2 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.namespace.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", &self.kind)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "name", &self.name)?;
        if let Some(value) = &self.namespace {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for Subject {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference, or a value for non-objects such as user and group names.",
          "properties": {
            "apiVersion": {
              "description": "APIVersion holds the API group and version of the referenced subject. Defaults to \"v1\" for ServiceAccount subjects. Defaults to \"rbac.authorization.k8s.io/v1alpha1\" for User and Group subjects.",
              "type": "string"
            },
            "kind": {
              "description": "Kind of object being referenced. Values defined by this API group are \"User\", \"Group\", and \"ServiceAccount\". If the Authorizer does not recognized the kind value, the Authorizer should report an error.",
              "type": "string"
            },
            "name": {
              "description": "Name of the object being referenced.",
              "type": "string"
            },
            "namespace": {
              "description": "Namespace of the referenced object.  If the object kind is non-namespace, such as \"User\" or \"Group\", and this value is not empty the Authorizer should report an error.",
              "type": "string"
            }
          },
          "required": [
            "kind",
            "name"
          ],
          "type": "object"
        })
    }
}
