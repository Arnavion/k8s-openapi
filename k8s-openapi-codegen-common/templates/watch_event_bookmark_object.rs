
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
