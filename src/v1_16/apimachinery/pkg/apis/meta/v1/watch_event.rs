// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent

/// Event represents a single event to a watched resource.
///
/// Object is:
///  * If Type is Added or Modified: the new state of the object.
///  * If Type is Deleted: the state of the object immediately before deletion.
///  * If Type is Error: *Status is recommended; other types may make sense
///    depending on context.
#[derive(Clone, Debug, PartialEq)]
pub enum WatchEvent<T> {
    Added(T),
    Deleted(T),
    Modified(T),
    Bookmark { resource_version: String },
    ErrorStatus(crate::apimachinery::pkg::apis::meta::v1::Status),
    ErrorOther(crate::apimachinery::pkg::runtime::RawExtension),
}

impl<'de, T> crate::serde::Deserialize<'de> for WatchEvent<T> where T: crate::serde::Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_type,
            Key_object,
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
                            "type" => Field::Key_type,
                            "object" => Field::Key_object,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        enum WatchEventType {
            Added,
            Deleted,
            Modified,
            Bookmark,
            Error,
        }

        impl<'de> crate::serde::Deserialize<'de> for WatchEventType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> crate::serde::de::Visitor<'de> for Visitor {
                    type Value = WatchEventType;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("watch event type")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "ADDED" => WatchEventType::Added,
                            "DELETED" => WatchEventType::Deleted,
                            "MODIFIED" => WatchEventType::Modified,
                            "BOOKMARK" => WatchEventType::Bookmark,
                            "ERROR" => WatchEventType::Error,
                            _ => return Err(crate::serde::de::Error::unknown_variant(
                                v,
                                &["ADDED", "DELETED", "MODIFIED", "BOOKMARK", "ERROR"],
                            )),
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> crate::serde::de::Visitor<'de> for Visitor<T> where T: crate::serde::Deserialize<'de> {
            type Value = WatchEvent<T>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("WatchEvent")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_type: Option<WatchEventType> = None;
                let mut value_object: Option<crate::serde_value::Value> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_type => value_type = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_object => value_object = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                let value_type = value_type.ok_or_else(|| crate::serde::de::Error::missing_field("type"))?;
                let value_object = value_object.ok_or_else(|| crate::serde::de::Error::missing_field("object"))?;

                Ok(match value_type {
                    WatchEventType::Added => {
                        let value_object = crate::serde_value::ValueDeserializer::new(value_object);
                        WatchEvent::Added(crate::serde::Deserialize::deserialize(value_object)?)
                    },
                    WatchEventType::Deleted => {
                        let value_object = crate::serde_value::ValueDeserializer::new(value_object);
                        WatchEvent::Deleted(crate::serde::Deserialize::deserialize(value_object)?)
                    },
                    WatchEventType::Modified => {
                        let value_object = crate::serde_value::ValueDeserializer::new(value_object);
                        WatchEvent::Modified(crate::serde::Deserialize::deserialize(value_object)?)
                    },
                    WatchEventType::Bookmark => {
                        let value_object = crate::serde_value::ValueDeserializer::new(value_object);
                        let value: BookmarkObject<'static> = crate::serde::Deserialize::deserialize(value_object)?;
                        WatchEvent::Bookmark {
                            resource_version: value.metadata.resource_version.into_owned(),
                        }
                    },
                    WatchEventType::Error => {
                        let is_status =
                            if let crate::serde_value::Value::Map(map) = &value_object {
                                matches!(map.get(&crate::serde_value::Value::String("kind".to_owned())), Some(crate::serde_value::Value::String(s)) if s == "Status")
                            }
                            else {
                                false
                            };
                        let value_object = crate::serde_value::ValueDeserializer::new(value_object);
                        if is_status {
                            WatchEvent::ErrorStatus(crate::serde::Deserialize::deserialize(value_object)?)
                        }
                        else {
                            WatchEvent::ErrorOther(crate::serde::Deserialize::deserialize(value_object)?)
                        }
                    },
                })
            }
        }

        deserializer.deserialize_struct(
            "WatchEvent",
            &[
                "type",
                "object",
            ],
            Visitor(Default::default()),
        )
    }
}

impl<T> crate::serde::Serialize for WatchEvent<T> where T: crate::serde::Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WatchEvent",
            2,
        )?;
        match self {
            WatchEvent::Added(object) => {
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ADDED")?;
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
            WatchEvent::Deleted(object) => {
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", "DELETED")?;
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
            WatchEvent::Modified(object) => {
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", "MODIFIED")?;
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
            WatchEvent::Bookmark { resource_version } => {
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", "BOOKMARK")?;
                let object = BookmarkObject {
                    metadata: BookmarkObjectMeta {
                        resource_version: std::borrow::Cow::Borrowed(&**resource_version),
                    },
                };
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
            WatchEvent::ErrorStatus(object) => {
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ERROR")?;
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
            WatchEvent::ErrorOther(object) => {
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ERROR")?;
                crate::serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[derive(Debug, PartialEq)]
struct BookmarkObject<'a> {
    metadata: BookmarkObjectMeta<'a>,
}

#[derive(Debug, PartialEq)]
struct BookmarkObjectMeta<'a> {
    resource_version: std::borrow::Cow<'a, str>,
}

impl<'de> crate::serde::Deserialize<'de> for BookmarkObject<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_metadata,
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
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = BookmarkObject<'static>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("BookmarkObject")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_metadata: Option<BookmarkObjectMeta<'static>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BookmarkObject {
                    metadata: value_metadata.ok_or_else(|| crate::serde::de::Error::missing_field("metadata"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "BookmarkObject",
            &[
                "metadata",
            ],
            Visitor,
        )
    }
}

impl<'de> crate::serde::Deserialize<'de> for BookmarkObjectMeta<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_resource_version,
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
                            "resourceVersion" => Field::Key_resource_version,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = BookmarkObjectMeta<'static>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ObjectMeta")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_resource_version: Option<String> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_resource_version => value_resource_version = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(BookmarkObjectMeta {
                    resource_version: std::borrow::Cow::Owned(value_resource_version.ok_or_else(|| crate::serde::de::Error::missing_field("resourceVersion"))?),
                })
            }
        }

        deserializer.deserialize_struct(
            "ObjectMeta",
            &[
                "resourceVersion",
            ],
            Visitor,
        )
    }
}

impl<'a> crate::serde::Serialize for BookmarkObject<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "BookmarkObject",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

impl<'a> crate::serde::Serialize for BookmarkObjectMeta<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ObjectMeta",
            1,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "resourceVersion", &self.resource_version)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}


#[cfg(feature = "schemars")]
impl<T> crate::schemars::JsonSchema for WatchEvent<T> {
    fn schema_name() -> String {
        "io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent".to_owned()
    }

    fn json_schema(__gen: &mut crate::schemars::gen::SchemaGenerator) -> crate::schemars::schema::Schema {
        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
            metadata: Some(Box::new(crate::schemars::schema::Metadata {
                description: Some("Event represents a single event to a watched resource.\n\nObject is:\n * If Type is Added or Modified: the new state of the object.\n * If Type is Deleted: the state of the object immediately before deletion.\n * If Type is Error: *Status is recommended; other types may make sense\n   depending on context.\n".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(crate::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "object".to_owned(),
                        __gen.subschema_for::<crate::apimachinery::pkg::runtime::RawExtension>(),
                    ),
                    (
                        "type".to_owned(),
                        crate::schemars::schema::Schema::Object(crate::schemars::schema::SchemaObject {
                            instance_type: Some(crate::schemars::schema::SingleOrVec::Single(Box::new(crate::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                ].into(),
                required: [
                    "object".to_owned(),
                    "type".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
