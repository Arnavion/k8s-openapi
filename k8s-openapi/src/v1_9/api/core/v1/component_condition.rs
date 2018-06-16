// Generated from definition io.k8s.api.core.v1.ComponentCondition

/// Information about the condition of a component.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ComponentCondition {
    /// Condition error code for a component. For example, a health check error code.
    pub error: Option<String>,

    /// Message about the condition for a component. For example, information about a health check.
    pub message: Option<String>,

    /// Status of the condition for a component. Valid values for "Healthy": "True", "False", or "Unknown".
    pub status: String,

    /// Type of condition for a component. Valid value: "Healthy"
    pub type_: String,
}

impl<'de> ::serde::Deserialize<'de> for ComponentCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_error,
            Key_message,
            Key_status,
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
                            "error" => Field::Key_error,
                            "message" => Field::Key_message,
                            "status" => Field::Key_status,
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
            type Value = ComponentCondition;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ComponentCondition")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_error: Option<String> = None;
                let mut value_message: Option<String> = None;
                let mut value_status: Option<String> = None;
                let mut value_type_: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_error => value_error = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_type_ => value_type_ = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ComponentCondition {
                    error: value_error,
                    message: value_message,
                    status: value_status.ok_or_else(|| ::serde::de::Error::missing_field("status"))?,
                    type_: value_type_.ok_or_else(|| ::serde::de::Error::missing_field("type"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ComponentCondition",
            &[
                "error",
                "message",
                "status",
                "type",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ComponentCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ComponentCondition",
            0 +
            self.error.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            1 +
            1,
        )?;
        if let Some(value) = &self.error {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "error", value)?;
        }
        if let Some(value) = &self.message {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", &self.status)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "type", &self.type_)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
