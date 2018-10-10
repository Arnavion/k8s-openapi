// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

/// Event represents a single event to a watched resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WatchEvent {
    /// Object is:
    ///  * If Type is Added or Modified: the new state of the object.
    ///  * If Type is Deleted: the state of the object immediately before deletion.
    ///  * If Type is Error: *Status is recommended; other types may make sense
    ///    depending on context.
    pub object: ::v1_12::apimachinery::pkg::runtime::RawExtension,

    pub type_: String,
}

impl<'de> ::serde::Deserialize<'de> for WatchEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_object,
            Key_type_,
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
                            "object" => Field::Key_object,
                            "type" => Field::Key_type_,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WatchEvent;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct WatchEvent")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_object: Option<::v1_12::apimachinery::pkg::runtime::RawExtension> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_object => value_object = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_type_ => value_type_ = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(WatchEvent {
                    object: value_object.ok_or_else(|| ::serde::de::Error::missing_field("object"))?,
                    type_: value_type_.ok_or_else(|| ::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "WatchEvent",
            &[
                "object",
                "type",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for WatchEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WatchEvent",
            0 +
            1 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "object", &self.object)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
