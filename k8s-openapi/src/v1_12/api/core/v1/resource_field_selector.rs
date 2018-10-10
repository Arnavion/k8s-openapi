// Generated from definition io.k8s.api.core.v1.ResourceFieldSelector

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceFieldSelector {
    /// Container name: required for volumes, optional for env vars
    pub container_name: Option<String>,

    /// Specifies the output format of the exposed resources, defaults to "1"
    pub divisor: Option<::v1_12::apimachinery::pkg::api::resource::Quantity>,

    /// Required: resource to select
    pub resource: String,
}

impl<'de> ::serde::Deserialize<'de> for ResourceFieldSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_container_name,
            Key_divisor,
            Key_resource,
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
                            "containerName" => Field::Key_container_name,
                            "divisor" => Field::Key_divisor,
                            "resource" => Field::Key_resource,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceFieldSelector;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ResourceFieldSelector")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_container_name: Option<String> = None;
                let mut value_divisor: Option<::v1_12::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_resource: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_container_name => value_container_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_divisor => value_divisor = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource => value_resource = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ResourceFieldSelector {
                    container_name: value_container_name,
                    divisor: value_divisor,
                    resource: value_resource.ok_or_else(|| ::serde::de::Error::missing_field("resource"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ResourceFieldSelector",
            &[
                "containerName",
                "divisor",
                "resource",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ResourceFieldSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ResourceFieldSelector",
            0 +
            self.container_name.as_ref().map_or(0, |_| 1) +
            self.divisor.as_ref().map_or(0, |_| 1) +
            1,
        )?;
        if let Some(value) = &self.container_name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "containerName", value)?;
        }
        if let Some(value) = &self.divisor {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "divisor", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "resource", &self.resource)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
