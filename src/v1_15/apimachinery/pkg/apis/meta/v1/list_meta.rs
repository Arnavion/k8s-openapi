// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta

/// ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListMeta {
    /// continue may be set if the user set a limit on the number of items returned, and indicates that the server has more data available. The value is opaque and may be used to issue another request to the endpoint that served this list to retrieve the next set of available objects. Continuing a consistent list may not be possible if the server configuration has changed or more than a few minutes have passed. The resourceVersion field returned when using this continue value will be identical to the value in the first response, unless you have received this token from an error message.
    pub continue_: Option<String>,

    /// remainingItemCount is the number of subsequent items in the list which are not included in this list response. If the list request contained label or field selectors, then the number of remaining items is unknown and the field will be left unset and omitted during serialization. If the list is complete (either because it is not chunking or because this is the last chunk), then there are no more remaining items and this field will be left unset and omitted during serialization. Servers older than v1.15 do not set this field. The intended use of the remainingItemCount is *estimating* the size of a collection. Clients should not rely on the remainingItemCount to be set or to be exact.
    ///
    /// This field is alpha and can be changed or removed without notice.
    pub remaining_item_count: Option<i64>,

    /// String that identifies the server's internal version of this object that can be used by clients to determine when objects have changed. Value must be treated as opaque by clients and passed unmodified back to the server. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency
    pub resource_version: Option<String>,

    /// selfLink is a URL representing this object. Populated by the system. Read-only.
    pub self_link: Option<String>,
}

impl<'de> crate::serde::Deserialize<'de> for ListMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_continue_,
            Key_remaining_item_count,
            Key_resource_version,
            Key_self_link,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "continue" => Field::Key_continue_,
                            "remainingItemCount" => Field::Key_remaining_item_count,
                            "resourceVersion" => Field::Key_resource_version,
                            "selfLink" => Field::Key_self_link,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = ListMeta;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ListMeta")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_continue_: Option<String> = None;
                let mut value_remaining_item_count: Option<i64> = None;
                let mut value_resource_version: Option<String> = None;
                let mut value_self_link: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_continue_ => value_continue_ = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_remaining_item_count => value_remaining_item_count = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_resource_version => value_resource_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_self_link => value_self_link = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ListMeta {
                    continue_: value_continue_,
                    remaining_item_count: value_remaining_item_count,
                    resource_version: value_resource_version,
                    self_link: value_self_link,
                })
            }
        }

        deserializer.deserialize_struct(
            "ListMeta",
            &[
                "continue",
                "remainingItemCount",
                "resourceVersion",
                "selfLink",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for ListMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ListMeta",
            self.continue_.as_ref().map_or(0, |_| 1) +
            self.remaining_item_count.as_ref().map_or(0, |_| 1) +
            self.resource_version.as_ref().map_or(0, |_| 1) +
            self.self_link.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.continue_ {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "continue", value)?;
        }
        if let Some(value) = &self.remaining_item_count {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "remainingItemCount", value)?;
        }
        if let Some(value) = &self.resource_version {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceVersion", value)?;
        }
        if let Some(value) = &self.self_link {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "selfLink", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schema")]
impl crate::Schema for ListMeta {
    fn schema() -> serde_json::Value {
        serde_json::json!({
          "description": "ListMeta describes metadata that synthetic resources must have, including lists and various status objects. A resource may have only one of {ObjectMeta, ListMeta}.",
          "properties": {
            "continue": {
              "description": "continue may be set if the user set a limit on the number of items returned, and indicates that the server has more data available. The value is opaque and may be used to issue another request to the endpoint that served this list to retrieve the next set of available objects. Continuing a consistent list may not be possible if the server configuration has changed or more than a few minutes have passed. The resourceVersion field returned when using this continue value will be identical to the value in the first response, unless you have received this token from an error message.",
              "type": "string"
            },
            "remainingItemCount": {
              "description": "remainingItemCount is the number of subsequent items in the list which are not included in this list response. If the list request contained label or field selectors, then the number of remaining items is unknown and the field will be left unset and omitted during serialization. If the list is complete (either because it is not chunking or because this is the last chunk), then there are no more remaining items and this field will be left unset and omitted during serialization. Servers older than v1.15 do not set this field. The intended use of the remainingItemCount is *estimating* the size of a collection. Clients should not rely on the remainingItemCount to be set or to be exact.\n\nThis field is alpha and can be changed or removed without notice.",
              "format": "int64",
              "type": "integer"
            },
            "resourceVersion": {
              "description": "String that identifies the server's internal version of this object that can be used by clients to determine when objects have changed. Value must be treated as opaque by clients and passed unmodified back to the server. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency",
              "type": "string"
            },
            "selfLink": {
              "description": "selfLink is a URL representing this object. Populated by the system. Read-only.",
              "type": "string"
            }
          },
          "type": "object"
        })
    }
}
