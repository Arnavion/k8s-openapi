
#[derive(Debug, PartialEq)]
struct BookmarkObject<'a, T> {{
    metadata: BookmarkObjectMeta<'a>,
    _resource: std::marker::PhantomData<T>,
}}

#[derive(Debug, PartialEq)]
struct BookmarkObjectMeta<'a> {{
    resource_version: std::borrow::Cow<'a, str>,
}}

impl<'de, T> serde::Deserialize<'de> for BookmarkObject<'static, T> where T: {local}Resource {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{
        #[allow(non_camel_case_types)]
        enum Field {{
            Key_api_version,
            Key_kind,
            Key_metadata,
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
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            _ => Field::Other,
                        }})
                    }}
                }}

                deserializer.deserialize_identifier(Visitor)
            }}
        }}

        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T> where T: {local}Resource {{
            type Value = BookmarkObject<'static, T>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str(<T as {local}Resource>::KIND)
            }}

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {{
                let mut value_metadata: Option<BookmarkObjectMeta<'static>> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {{
                    match key {{
                        Field::Key_api_version => {{
                            let value_api_version: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <T as {local}Resource>::API_VERSION {{
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_api_version), &<T as {local}Resource>::API_VERSION));
                            }}
                        }},
                        Field::Key_kind => {{
                            let value_kind: String = serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <T as {local}Resource>::KIND {{
                                return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&value_kind), &<T as {local}Resource>::KIND));
                            }}
                        }},
                        Field::Key_metadata => value_metadata = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => {{ let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; }},
                    }}
                }}

                Ok(BookmarkObject {{
                    metadata: value_metadata.ok_or_else(|| serde::de::Error::missing_field("metadata"))?,
                    _resource: Default::default()
                }})
            }}
        }}

        deserializer.deserialize_struct(
            <T as {local}Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "metadata",
            ],
            Visitor(Default::default()),
        )
    }}
}}

impl<'de> serde::Deserialize<'de> for BookmarkObjectMeta<'static> {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {{
        #[allow(non_camel_case_types)]
        enum Field {{
            Key_resource_version,
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
                            "resourceVersion" => Field::Key_resource_version,
                            _ => Field::Other,
                        }})
                    }}
                }}

                deserializer.deserialize_identifier(Visitor)
            }}
        }}

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {{
            type Value = BookmarkObjectMeta<'static>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str("ObjectMeta")
            }}

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: serde::de::MapAccess<'de> {{
                let mut value_resource_version: Option<String> = None;

                while let Some(key) = serde::de::MapAccess::next_key::<Field>(&mut map)? {{
                    match key {{
                        Field::Key_resource_version => value_resource_version = serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => {{ let _: serde::de::IgnoredAny = serde::de::MapAccess::next_value(&mut map)?; }},
                    }}
                }}

                Ok(BookmarkObjectMeta {{
                    resource_version: std::borrow::Cow::Owned(value_resource_version.ok_or_else(|| serde::de::Error::missing_field("resourceVersion"))?),
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

impl<'a, T> serde::Serialize for BookmarkObject<'a, T> where T: {local}Resource {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{
        let mut state = serializer.serialize_struct(
            <T as {local}Resource>::KIND,
            3,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <T as {local}Resource>::API_VERSION)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <T as {local}Resource>::KIND)?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        serde::ser::SerializeStruct::end(state)
    }}
}}

impl<'a> serde::Serialize for BookmarkObjectMeta<'a> {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{
        let mut state = serializer.serialize_struct(
            "ObjectMeta",
            1,
        )?;
        serde::ser::SerializeStruct::serialize_field(&mut state, "resourceVersion", &self.resource_version)?;
        serde::ser::SerializeStruct::end(state)
    }}
}}
