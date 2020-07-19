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
    ErrorStatus(crate::apimachinery::pkg::apis::meta::v1::Status),
    ErrorOther(crate::apimachinery::pkg::runtime::RawExtension),
}

impl<'de, T> serde::Deserialize<'de> for WatchEvent<T> where T: serde::Deserialize<'de> + crate::Resource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_type,
            Key_object,
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
            Error,
        }

        impl<'de> serde::Deserialize<'de> for WatchEventType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = WatchEventType;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("watch event type")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                        Ok(match v {
                            "ADDED" => WatchEventType::Added,
                            "DELETED" => WatchEventType::Deleted,
                            "MODIFIED" => WatchEventType::Modified,
                            "ERROR" => WatchEventType::Error,
                            _ => return Err(serde::de::Error::unknown_variant(
                                v,
                                &["ADDED", "DELETED", "MODIFIED", "ERROR"],
                            )),
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T> where T: serde::Deserialize<'de> + crate::Resource {
            type Value = WatchEvent<T>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("WatchEvent")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {
                let mut value_type: Option<WatchEventType> = None;
                let mut value_object: Option<serde_value::Value> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_type => value_type = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_object => value_object = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                let value_type = value_type.ok_or_else(|| serde::de::Error::missing_field("type"))?;
                let value_object = value_object.ok_or_else(|| serde::de::Error::missing_field("object"))?;

                Ok(match value_type {
                    WatchEventType::Added => {
                        let value_object = serde_value::ValueDeserializer::new(value_object);
                        WatchEvent::Added(serde::Deserialize::deserialize(value_object)?)
                    },
                    WatchEventType::Deleted => {
                        let value_object = serde_value::ValueDeserializer::new(value_object);
                        WatchEvent::Deleted(serde::Deserialize::deserialize(value_object)?)
                    },
                    WatchEventType::Modified => {
                        let value_object = serde_value::ValueDeserializer::new(value_object);
                        WatchEvent::Modified(serde::Deserialize::deserialize(value_object)?)
                    },

                    WatchEventType::Error => {
                        let is_status =
                            if let serde_value::Value::Map(map) = &value_object {
                                matches!(map.get(&serde_value::Value::String("kind".to_owned())), Some(serde_value::Value::String(s)) if s == "Status")
                            }
                            else {
                                false
                            };
                        let value_object = serde_value::ValueDeserializer::new(value_object);
                        if is_status {
                            WatchEvent::ErrorStatus(serde::Deserialize::deserialize(value_object)?)
                        }
                        else {
                            WatchEvent::ErrorOther(serde::Deserialize::deserialize(value_object)?)
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

impl<T> serde::Serialize for WatchEvent<T> where T: serde::Serialize + crate::Resource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct(
            "WatchEvent",
            2,
        )?;
        match self {
            WatchEvent::Added(object) => {
                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ADDED")?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
            WatchEvent::Deleted(object) => {
                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "DELETED")?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
            WatchEvent::Modified(object) => {
                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "MODIFIED")?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
            WatchEvent::ErrorStatus(object) => {
                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ERROR")?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
            WatchEvent::ErrorOther(object) => {
                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ERROR")?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            },
        }
        serde::ser::SerializeStruct::end(state)
    }
}
