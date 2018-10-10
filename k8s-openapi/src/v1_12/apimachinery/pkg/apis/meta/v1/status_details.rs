// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.StatusDetails

/// StatusDetails is a set of additional properties that MAY be set by the server to provide additional information about a response. The Reason field of a Status object defines what attributes will be set. Clients must ignore fields that do not match the defined type of each attribute, and should assume that any attribute may be empty, invalid, or under defined.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatusDetails {
    /// The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.
    pub causes: Option<Vec<::v1_12::apimachinery::pkg::apis::meta::v1::StatusCause>>,

    /// The group attribute of the resource associated with the status StatusReason.
    pub group: Option<String>,

    /// The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described).
    pub name: Option<String>,

    /// If specified, the time in seconds before the operation should be retried. Some errors may indicate the client must take an alternate action - for those errors this field may indicate how long to wait before taking the alternate action.
    pub retry_after_seconds: Option<i32>,

    /// UID of the resource. (when there is a single resource which can be described). More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    pub uid: Option<String>,
}

impl<'de> ::serde::Deserialize<'de> for StatusDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_causes,
            Key_group,
            Key_kind,
            Key_name,
            Key_retry_after_seconds,
            Key_uid,
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
                            "causes" => Field::Key_causes,
                            "group" => Field::Key_group,
                            "kind" => Field::Key_kind,
                            "name" => Field::Key_name,
                            "retryAfterSeconds" => Field::Key_retry_after_seconds,
                            "uid" => Field::Key_uid,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StatusDetails;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct StatusDetails")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_causes: Option<Vec<::v1_12::apimachinery::pkg::apis::meta::v1::StatusCause>> = None;
                let mut value_group: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_name: Option<String> = None;
                let mut value_retry_after_seconds: Option<i32> = None;
                let mut value_uid: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_causes => value_causes = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_group => value_group = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_name => value_name = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_retry_after_seconds => value_retry_after_seconds = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_uid => value_uid = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StatusDetails {
                    causes: value_causes,
                    group: value_group,
                    kind: value_kind,
                    name: value_name,
                    retry_after_seconds: value_retry_after_seconds,
                    uid: value_uid,
                })
            }
        }

        deserializer.deserialize_struct(
            "StatusDetails",
            &[
                "causes",
                "group",
                "kind",
                "name",
                "retryAfterSeconds",
                "uid",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for StatusDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StatusDetails",
            0 +
            self.causes.as_ref().map_or(0, |_| 1) +
            self.group.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.name.as_ref().map_or(0, |_| 1) +
            self.retry_after_seconds.as_ref().map_or(0, |_| 1) +
            self.uid.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.causes {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "causes", value)?;
        }
        if let Some(value) = &self.group {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "group", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.name {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "name", value)?;
        }
        if let Some(value) = &self.retry_after_seconds {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "retryAfterSeconds", value)?;
        }
        if let Some(value) = &self.uid {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "uid", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
