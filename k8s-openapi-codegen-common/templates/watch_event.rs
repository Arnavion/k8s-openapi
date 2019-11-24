enum {type_name}<T> {{
    Added(T),
    Deleted(T),
    Modified(T),
    {bookmark_def}ErrorStatus({error_status_rust_type}),
    ErrorOther({error_other_rust_type}),
}}

impl<'de, T> serde::Deserialize<'de> for {type_name}<T> where T: serde::Deserialize<'de> {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{
        #[allow(non_camel_case_types)]
        enum Field {{
            Key_type,
            Key_object,
            Other,
        }}

        impl<'de> serde::Deserialize<'de> for Field {{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {{
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                        f.write_str("field identifier")
                    }}

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {{
                        Ok(match v {{
                            "type" => Field::Key_type,
                            "object" => Field::Key_object,
                            _ => Field::Other,
                        }})
                    }}
                }}

                deserializer.deserialize_identifier(Visitor)
            }}
        }}

        enum WatchEventType {{
            Added,
            Deleted,
            Modified,
            {bookmark_event_type}Error,
        }}

        impl<'de> serde::Deserialize<'de> for WatchEventType {{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {{
                    type Value = WatchEventType;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                        f.write_str("watch event type")
                    }}

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {{
                        Ok(match v {{
                            "ADDED" => WatchEventType::Added,
                            "DELETED" => WatchEventType::Deleted,
                            "MODIFIED" => WatchEventType::Modified,
                            {bookmark_type_match_arm}"ERROR" => WatchEventType::Error,
                            _ => return Err(serde::de::Error::unknown_variant(
                                v,
                                &["ADDED", "DELETED", "MODIFIED", {bookmark_type_value}"ERROR"],
                            )),
                        }})
                    }}
                }}

                deserializer.deserialize_identifier(Visitor)
            }}
        }}

        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T> where T: serde::Deserialize<'de> {{
            type Value = {type_name}<T>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str({type_name:?})
            }}

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {{
                let mut value_type: Option<WatchEventType> = None;
                let mut value_object: Option<serde_value::Value> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {{
                    match key {{
                        Field::Key_type => value_type = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_object => value_object = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => {{ let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; }},
                    }}
                }}

                let value_type = value_type.ok_or_else(|| serde::de::Error::missing_field("type"))?;
                let value_object = value_object.ok_or_else(|| serde::de::Error::missing_field("object"))?;

                Ok(match value_type {{
                    WatchEventType::Added => {{
                        let value_object = serde_value::ValueDeserializer::new(value_object);
                        {type_name}::Added(serde::Deserialize::deserialize(value_object)?)
                    }},
                    WatchEventType::Deleted => {{
                        let value_object = serde_value::ValueDeserializer::new(value_object);
                        {type_name}::Deleted(serde::Deserialize::deserialize(value_object)?)
                    }},
                    WatchEventType::Modified => {{
                        let value_object = serde_value::ValueDeserializer::new(value_object);
                        {type_name}::Modified(serde::Deserialize::deserialize(value_object)?)
                    }},
{bookmark_value_match_arm}
                    WatchEventType::Error => {{
                        let is_status =
                            if let serde_value::Value::Map(map) = &value_object {{
                                match map.get(&serde_value::Value::String("kind".to_owned())) {{
                                    Some(serde_value::Value::String(s)) if s == "Status" => true,
                                    _ => false,
                                }}
                            }}
                            else {{
                                false
                            }};
                        let value_object = serde_value::ValueDeserializer::new(value_object);
                        if is_status {{
                            {type_name}::ErrorStatus(serde::Deserialize::deserialize(value_object)?)
                        }}
                        else {{
                            {type_name}::ErrorOther(serde::Deserialize::deserialize(value_object)?)
                        }}
                    }},
                }})
            }}
        }}

        deserializer.deserialize_struct(
            {type_name:?},
            &[
                "type",
                "object",
            ],
            Visitor(Default::default()),
        )
    }}
}}

impl<T> serde::Serialize for {type_name}<T> where T: serde::Serialize {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{
        let mut state = serializer.serialize_struct(
            {type_name:?},
            2,
        )?;
        match self {{
            {type_name}::Added(object) => {{
                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ADDED")?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
            {type_name}::Deleted(object) => {{
                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "DELETED")?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
            {type_name}::Modified(object) => {{
                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "MODIFIED")?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
            {bookmark_serialize}{type_name}::ErrorStatus(object) => {{
                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ERROR")?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
            {type_name}::ErrorOther(object) => {{
                serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ERROR")?;
                serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
        }}
        serde::ser::SerializeStruct::end(state)
    }}
}}