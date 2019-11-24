// Generated from definition io.k8s.api.core.v1.ObjectReference

/// ObjectReference contains enough information to let you inspect or modify the referred object.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectReference {
    /// API version of the referent.
    pub api_version: Option<String>,

    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers\[2\]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers\[2\]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object.
    pub field_path: Option<String>,

    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: Option<String>,

    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    pub namespace: Option<String>,

    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    pub resource_version: Option<String>,

    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    pub uid: Option<String>,
}

impl<'de> serde::Deserialize<'de> for ObjectReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_field_path,
            Key_kind,
            Key_name,
            Key_namespace,
            Key_resource_version,
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
                            "fieldPath" => Field::Key_field_path,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "namespace" => Field::Key_namespace,
                            "resourceVersion" => Field::Key_resource_version,
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
            type Value = ObjectReference;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ObjectReference")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_field_path: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_namespace: Option<String> = None;
                let mut value_resource_version: Option<String> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_field_path => value_field_path = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_namespace => value_namespace = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_version => value_resource_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ObjectReference {
                    api_version: value_api_version,
                    field_path: value_field_path,
                    kind: value_kind,
                    name: value_name,
                    namespace: value_namespace,
                    resource_version: value_resource_version,
                    uid: value_uid,
                })
            }
        }

        deserializer.deserialize_struct(
            "ObjectReference",
            &[
                "apiVersion",
                "fieldPath",
                "kind",
                "name",
                "namespace",
                "resourceVersion",
                "uid",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for ObjectReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ObjectReference",
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.field_path.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.namespace.as_ref().map_or(0, |_| 1) +
            self.resource_version.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.field_path {
            serde::ser::SerializeStruct::serialize_field(&mut state, "fieldPath", value)?;
        }
        if let Some(value) = &self.kind {
            serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.name {
            serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.namespace {
            serde::ser::SerializeStruct::serialize_field(&mut state, "namespace", value)?;
        }
        if let Some(value) = &self.resource_version {
            serde::ser::SerializeStruct::serialize_field(&mut state, "resourceVersion", value)?;
        }
        if let Some(value) = &self.uid {
            serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
