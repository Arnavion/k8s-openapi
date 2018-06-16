// Generated from definition io.k8s.api.apps.v1beta2.ControllerRevision

/// ControllerRevision implements an immutable snapshot of state data. Clients are responsible for serializing and deserializing the objects that contain their internal state. Once a ControllerRevision has been successfully created, it can not be updated. The API Server will fail validation of all requests that attempt to mutate the Data field. ControllerRevisions may, however, be deleted. Note that, due to its use by both the DaemonSet and StatefulSet controllers for update and rollback, this object is beta. However, it may be subject to name and representation changes in future releases, and clients should not depend on its stability. It is primarily for internal use by controllers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ControllerRevision {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Data is the serialized representation of the state.
    pub data: Option<::v1_8::apimachinery::pkg::runtime::RawExtension>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Revision indicates the revision of the state represented by Data.
    pub revision: i64,
}

impl<'de> ::serde::Deserialize<'de> for ControllerRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_data,
            Key_kind,
            Key_metadata,
            Key_revision,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "data" => Field::Key_data,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "revision" => Field::Key_revision,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ControllerRevision;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ControllerRevision")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_data: Option<::v1_8::apimachinery::pkg::runtime::RawExtension> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_revision: Option<i64> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_data => value_data = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_revision => value_revision = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ControllerRevision {
                    api_version: value_api_version,
                    data: value_data,
                    kind: value_kind,
                    metadata: value_metadata,
                    revision: value_revision.ok_or_else(|| ::serde::de::Error::missing_field("revision"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ControllerRevision",
            &[
                "apiVersion",
                "data",
                "kind",
                "metadata",
                "revision",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ControllerRevision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ControllerRevision",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.data.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.data {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "data", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "revision", &self.revision)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
