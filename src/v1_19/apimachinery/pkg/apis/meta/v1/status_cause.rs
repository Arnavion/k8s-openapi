// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.StatusCause

/// StatusCause provides more information about an api.Status failure, including cases when multiple errors are encountered.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatusCause {
    /// The field of the resource that has caused this error, as named by its JSON serialization. May include dot and postfix notation for nested attributes. Arrays are zero-indexed.  Fields may appear more than once in an array of causes due to fields having multiple errors. Optional.
    ///
    /// Examples:
    ///   "name" - the field "name" on the current resource
    ///   "items\[0\].name" - the field "name" on the first array entry in "items"
    pub field: Option<String>,

    /// A human-readable description of the cause of the error.  This field may be presented as-is to a reader.
    pub message: Option<String>,

    /// A machine-readable description of the cause of the error. If this value is empty there is no information available.
    pub reason: Option<String>,
}

impl<'de> serde::Deserialize<'de> for StatusCause {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_field,
            Key_message,
            Key_reason,
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
                            "field" => Field::Key_field,
                            "message" => Field::Key_message,
                            "reason" => Field::Key_reason,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StatusCause;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("StatusCause")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_field: Option<String> = None;
                let mut value_message: Option<String> = None;
                let mut value_reason: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_field => value_field = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatusCause {
                    field: value_field,
                    message: value_message,
                    reason: value_reason,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatusCause",
            &[
                "field",
                "message",
                "reason",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for StatusCause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatusCause",
            self.field.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.field {
            serde::ser::SerializeStruct::serialize_field(&mut state, "field", value)?;
        }
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.reason {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
