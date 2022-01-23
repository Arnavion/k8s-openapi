enum {type_name}<T> {{
    Added(T),
    Deleted(T),
    Modified(T),
    Bookmark {{ resource_version: String }},
    ErrorStatus({error_status_rust_type}),
    ErrorOther({error_other_rust_type}),
}}

impl<'de, T> {local}serde::Deserialize<'de> for {type_name}<T> where T: {local}serde::Deserialize<'de> {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
        #[allow(non_camel_case_types)]
        enum Field {{
            Key_type,
            Key_object,
            Other,
        }}

        impl<'de> {local}serde::Deserialize<'de> for Field {{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
                struct Visitor;

                impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                        f.write_str("field identifier")
                    }}

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
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
            Bookmark,
            Error,
        }}

        impl<'de> {local}serde::Deserialize<'de> for WatchEventType {{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
                struct Visitor;

                impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
                    type Value = WatchEventType;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                        f.write_str("watch event type")
                    }}

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                        Ok(match v {{
                            "ADDED" => WatchEventType::Added,
                            "DELETED" => WatchEventType::Deleted,
                            "MODIFIED" => WatchEventType::Modified,
                            "BOOKMARK" => WatchEventType::Bookmark,
                            "ERROR" => WatchEventType::Error,
                            _ => return Err({local}serde::de::Error::unknown_variant(
                                v,
                                &["ADDED", "DELETED", "MODIFIED", "BOOKMARK", "ERROR"],
                            )),
                        }})
                    }}
                }}

                deserializer.deserialize_identifier(Visitor)
            }}
        }}

        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> {local}serde::de::Visitor<'de> for Visitor<T> where T: {local}serde::Deserialize<'de> {{
            type Value = {type_name}<T>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str({type_name:?})
            }}

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: {local}serde::de::MapAccess<'de> {{
                let mut value_type: Option<WatchEventType> = None;
                let mut value_object: Option<{local}serde_value::Value> = None;

                while let Some(key) = {local}serde::de::MapAccess::next_key::<Field>(&mut map)? {{
                    match key {{
                        Field::Key_type => value_type = {local}serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_object => value_object = {local}serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => {{ let _: {local}serde::de::IgnoredAny = {local}serde::de::MapAccess::next_value(&mut map)?; }},
                    }}
                }}

                let value_type = value_type.ok_or_else(|| {local}serde::de::Error::missing_field("type"))?;
                let value_object = value_object.ok_or_else(|| {local}serde::de::Error::missing_field("object"))?;

                Ok(match value_type {{
                    WatchEventType::Added => {{
                        let value_object = {local}serde_value::ValueDeserializer::new(value_object);
                        {type_name}::Added({local}serde::Deserialize::deserialize(value_object)?)
                    }},
                    WatchEventType::Deleted => {{
                        let value_object = {local}serde_value::ValueDeserializer::new(value_object);
                        {type_name}::Deleted({local}serde::Deserialize::deserialize(value_object)?)
                    }},
                    WatchEventType::Modified => {{
                        let value_object = {local}serde_value::ValueDeserializer::new(value_object);
                        {type_name}::Modified({local}serde::Deserialize::deserialize(value_object)?)
                    }},
                    WatchEventType::Bookmark => {{
                        let value_object = {local}serde_value::ValueDeserializer::new(value_object);
                        let value: BookmarkObject<'static> = {local}serde::Deserialize::deserialize(value_object)?;
                        {type_name}::Bookmark {{
                            resource_version: value.metadata.resource_version.into_owned(),
                        }}
                    }},
                    WatchEventType::Error => {{
                        let is_status =
                            if let {local}serde_value::Value::Map(map) = &value_object {{
                                matches!(map.get(&{local}serde_value::Value::String("kind".to_owned())), Some({local}serde_value::Value::String(s)) if s == "Status")
                            }}
                            else {{
                                false
                            }};
                        let value_object = {local}serde_value::ValueDeserializer::new(value_object);
                        if is_status {{
                            {type_name}::ErrorStatus({local}serde::Deserialize::deserialize(value_object)?)
                        }}
                        else {{
                            {type_name}::ErrorOther({local}serde::Deserialize::deserialize(value_object)?)
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

impl<T> {local}serde::Serialize for {type_name}<T> where T: {local}serde::Serialize {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: {local}serde::Serializer {{
        let mut state = serializer.serialize_struct(
            {type_name:?},
            2,
        )?;
        match self {{
            {type_name}::Added(object) => {{
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ADDED")?;
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
            {type_name}::Deleted(object) => {{
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "type", "DELETED")?;
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
            {type_name}::Modified(object) => {{
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "type", "MODIFIED")?;
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
            {type_name}::Bookmark {{ resource_version }} => {{
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "type", "BOOKMARK")?;
                let object = BookmarkObject {{
                    metadata: BookmarkObjectMeta {{
                        resource_version: std::borrow::Cow::Borrowed(&**resource_version),
                    }},
                }};
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
            {type_name}::ErrorStatus(object) => {{
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ERROR")?;
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
            {type_name}::ErrorOther(object) => {{
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "type", "ERROR")?;
                {local}serde::ser::SerializeStruct::serialize_field(&mut state, "object", &object)?;
            }},
        }}
        {local}serde::ser::SerializeStruct::end(state)
    }}
}}

#[derive(Debug, PartialEq)]
struct BookmarkObject<'a> {{
    metadata: BookmarkObjectMeta<'a>,
}}

#[derive(Debug, PartialEq)]
struct BookmarkObjectMeta<'a> {{
    resource_version: std::borrow::Cow<'a, str>,
}}

impl<'de> {local}serde::Deserialize<'de> for BookmarkObject<'static> {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
        #[allow(non_camel_case_types)]
        enum Field {{
            Key_metadata,
            Other,
        }}

        impl<'de> {local}serde::Deserialize<'de> for Field {{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
                struct Visitor;

                impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                        f.write_str("field identifier")
                    }}

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                        Ok(match v {{
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        }})
                    }}
                }}

                deserializer.deserialize_identifier(Visitor)
            }}
        }}

        struct Visitor;

        impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
            type Value = BookmarkObject<'static>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str("BookmarkObject")
            }}

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: {local}serde::de::MapAccess<'de> {{
                let mut value_metadata: Option<BookmarkObjectMeta<'static>> = None;

                while let Some(key) = {local}serde::de::MapAccess::next_key::<Field>(&mut map)? {{
                    match key {{
                        Field::Key_metadata => value_metadata = {local}serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => {{ let _: {local}serde::de::IgnoredAny = {local}serde::de::MapAccess::next_value(&mut map)?; }},
                    }}
                }}

                Ok(BookmarkObject {{
                    metadata: value_metadata.ok_or_else(|| {local}serde::de::Error::missing_field("metadata"))?,
                }})
            }}
        }}

        deserializer.deserialize_struct(
            "BookmarkObject",
            &[
                "metadata",
            ],
            Visitor,
        )
    }}
}}

impl<'de> {local}serde::Deserialize<'de> for BookmarkObjectMeta<'static> {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
        #[allow(non_camel_case_types)]
        enum Field {{
            Key_resource_version,
            Other,
        }}

        impl<'de> {local}serde::Deserialize<'de> for Field {{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
                struct Visitor;

                impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                        f.write_str("field identifier")
                    }}

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                        Ok(match v {{
                            "resourceVersion" => Field::Key_resource_version,
                            _ => Field::Other,
                        }})
                    }}
                }}

                deserializer.deserialize_identifier(Visitor)
            }}
        }}

        struct Visitor;

        impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
            type Value = BookmarkObjectMeta<'static>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str("ObjectMeta")
            }}

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: {local}serde::de::MapAccess<'de> {{
                let mut value_resource_version: Option<String> = None;

                while let Some(key) = {local}serde::de::MapAccess::next_key::<Field>(&mut map)? {{
                    match key {{
                        Field::Key_resource_version => value_resource_version = {local}serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => {{ let _: {local}serde::de::IgnoredAny = {local}serde::de::MapAccess::next_value(&mut map)?; }},
                    }}
                }}

                Ok(BookmarkObjectMeta {{
                    resource_version: std::borrow::Cow::Owned(value_resource_version.ok_or_else(|| {local}serde::de::Error::missing_field("resourceVersion"))?),
                }})
            }}
        }}

        deserializer.deserialize_struct(
            "ObjectMeta",
            &[
                "resourceVersion",
            ],
            Visitor,
        )
    }}
}}

impl<'a> {local}serde::Serialize for BookmarkObject<'a> {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: {local}serde::Serializer {{
        let mut state = serializer.serialize_struct(
            "BookmarkObject",
            1,
        )?;
        {local}serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        {local}serde::ser::SerializeStruct::end(state)
    }}
}}

impl<'a> {local}serde::Serialize for BookmarkObjectMeta<'a> {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: {local}serde::Serializer {{
        let mut state = serializer.serialize_struct(
            "ObjectMeta",
            1,
        )?;
        {local}serde::ser::SerializeStruct::serialize_field(&mut state, "resourceVersion", &self.resource_version)?;
        {local}serde::ser::SerializeStruct::end(state)
    }}
}}
