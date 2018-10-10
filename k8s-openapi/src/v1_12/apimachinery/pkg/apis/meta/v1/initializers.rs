// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Initializers

/// Initializers tracks the progress of initialization.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Initializers {
    /// Pending is a list of initializers that must execute in order before this object is visible. When the last pending initializer is removed, and no failing result is set, the initializers struct will be set to nil and the object is considered as initialized and visible to all clients.
    pub pending: Vec<::v1_12::apimachinery::pkg::apis::meta::v1::Initializer>,

    /// If result is set with the Failure field, the object will be persisted to storage and then deleted, ensuring that other clients can observe the deletion.
    pub result: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Status>,
}

impl<'de> ::serde::Deserialize<'de> for Initializers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_pending,
            Key_result,
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
                            "pending" => Field::Key_pending,
                            "result" => Field::Key_result,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Initializers;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Initializers")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_pending: Option<Vec<::v1_12::apimachinery::pkg::apis::meta::v1::Initializer>> = None;
                let mut value_result: Option<::v1_12::apimachinery::pkg::apis::meta::v1::Status> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_pending => value_pending = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_result => value_result = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Initializers {
                    pending: value_pending.ok_or_else(|| ::serde::de::Error::missing_field("pending"))?,
                    result: value_result,
                })
            }
        }

        deserializer.deserialize_struct(
            "Initializers",
            &[
                "pending",
                "result",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Initializers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Initializers",
            0 +
            1 +
            self.result.as_ref().map_or(0, |_| 1),
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "pending", &self.pending)?;
        if let Some(value) = &self.result {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "result", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
