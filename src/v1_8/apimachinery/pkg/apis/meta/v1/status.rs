// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Status

/// Status is a return value for calls that don't return other objects.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Status {
    /// Suggested HTTP return code for this status, 0 if not set.
    pub code: Option<i32>,

    /// Extended data associated with the reason.  Each reason may define its own extended details. This field is optional and the data returned is not guaranteed to conform to any schema except that defined by the reason type.
    pub details: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::StatusDetails>,

    /// A human-readable description of the status of this operation.
    pub message: Option<String>,

    /// Standard list metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ListMeta>,

    /// A machine-readable description of why this operation is in the "Failure" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it.
    pub reason: Option<String>,

    /// Status of the operation. One of: "Success" or "Failure". More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<String>,
}

impl crate::Resource for Status {
    fn api_version() -> &'static str {
        "v1"
    }

    fn group() -> &'static str {
        ""
    }

    fn kind() -> &'static str {
        "Status"
    }

    fn version() -> &'static str {
        "v1"
    }
}

impl crate::Metadata for Status {
    type Ty = crate::v1_8::apimachinery::pkg::apis::meta::v1::ListMeta;

    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
}

impl<'de> serde::Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_code,
            Key_details,
            Key_message,
            Key_metadata,
            Key_reason,
            Key_status,
            Other,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "code" => Field::Key_code,
                            "details" => Field::Key_details,
                            "message" => Field::Key_message,
                            "metadata" => Field::Key_metadata,
                            "reason" => Field::Key_reason,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Status;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "struct Status")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_code: Option<i32> = None;
                let mut value_details: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::StatusDetails> = None;
                let mut value_message: Option<String> = None;
                let mut value_metadata: Option<crate::v1_8::apimachinery::pkg::apis::meta::v1::ListMeta> = None;
                let mut value_reason: Option<String> = None;
                let mut value_status: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::api_version() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::api_version()));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::kind() {
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::kind()));
                            }
                        },
                        Field::Key_code => value_code = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_details => value_details = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_message => value_message = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_reason => value_reason = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Status {
                    code: value_code,
                    details: value_details,
                    message: value_message,
                    metadata: value_metadata,
                    reason: value_reason,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Status",
            &[
                "apiVersion",
                "kind",
                "code",
                "details",
                "message",
                "metadata",
                "reason",
                "status",
            ],
            Visitor,
        )
    }
}

impl serde::Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Status",
            2 +
            self.code.as_ref().map_or(0, |_| 1) +
            self.details.as_ref().map_or(0, |_| 1) +
            self.message.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.reason.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::api_version())?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::kind())?;
        if let Some(value) = &self.code {
            serde::ser::SerializeStruct::serialize_field(&mut state, "code", value)?;
        }
        if let Some(value) = &self.details {
            serde::ser::SerializeStruct::serialize_field(&mut state, "details", value)?;
        }
        if let Some(value) = &self.message {
            serde::ser::SerializeStruct::serialize_field(&mut state, "message", value)?;
        }
        if let Some(value) = &self.metadata {
            serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.reason {
            serde::ser::SerializeStruct::serialize_field(&mut state, "reason", value)?;
        }
        if let Some(value) = &self.status {
            serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        serde::ser::SerializeStruct::end(state)
    }
}
