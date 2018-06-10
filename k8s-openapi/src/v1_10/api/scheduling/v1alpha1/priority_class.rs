// Generated from definition io.k8s.api.scheduling.v1alpha1.PriorityClass

/// PriorityClass defines mapping from a priority class name to the priority integer value. The value can be any valid integer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PriorityClass {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// description is an arbitrary string that usually provides guidelines on when this priority class should be used.
    pub description: Option<String>,

    /// globalDefault specifies whether this PriorityClass should be considered as the default priority for pods that do not have any priority class. Only one PriorityClass can be marked as `globalDefault`. However, if more than one PriorityClasses exists with their `globalDefault` field set to true, the smallest value of such global default PriorityClasses will be used as the default priority.
    pub global_default: Option<bool>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// The value of this priority class. This is the actual priority that pods receive when they have the name of this class in their pod spec.
    pub value: i32,
}

impl<'de> ::serde::Deserialize<'de> for PriorityClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_description,
            Key_global_default,
            Key_kind,
            Key_metadata,
            Key_value,
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
                            "description" => Field::Key_description,
                            "globalDefault" => Field::Key_global_default,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "value" => Field::Key_value,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PriorityClass;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PriorityClass")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_description: Option<String> = None;
                let mut value_global_default: Option<bool> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_value: Option<i32> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_description => value_description = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_global_default => value_global_default = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_value => value_value = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(PriorityClass {
                    api_version: value_api_version,
                    description: value_description,
                    global_default: value_global_default,
                    kind: value_kind,
                    metadata: value_metadata,
                    value: value_value.ok_or_else(|| ::serde::de::Error::missing_field("value"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "PriorityClass",
            &[
                "apiVersion",
                "description",
                "globalDefault",
                "kind",
                "metadata",
                "value",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for PriorityClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PriorityClass",
            0 +
            (if self.api_version.is_some() { 1 } else { 0 }) +
            (if self.description.is_some() { 1 } else { 0 }) +
            (if self.global_default.is_some() { 1 } else { 0 }) +
            (if self.kind.is_some() { 1 } else { 0 }) +
            (if self.metadata.is_some() { 1 } else { 0 }) +
            1,
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.description {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "description", value)?;
        }
        if let Some(value) = &self.global_default {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "globalDefault", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "value", &self.value)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
