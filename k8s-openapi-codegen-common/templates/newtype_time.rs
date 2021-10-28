struct {type_name}({vis}{inner_type_name});

impl<'de> {local}serde::Deserialize<'de> for {type_name} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
        struct Visitor;

        impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
            type Value = {type_name};

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str({type_name:?})
            }}

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: {local}serde::Deserializer<'de> {{
                struct Visitor;

                impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
                    type Value = {local}time::OffsetDateTime;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                        f.write_str("OffsetDateTime")
                    }}

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                        {local}time::OffsetDateTime::parse(s, &{local}time::format_description::well_known::Rfc3339).map_err(serde::de::Error::custom)
                    }}
                }}

                Ok({type_name}(deserializer.deserialize_str(Visitor)?))
            }}
        }}

        deserializer.deserialize_newtype_struct({type_name:?}, Visitor)
    }}
}}

impl {local}serde::Serialize for {type_name} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: {local}serde::Serializer {{
        serializer.serialize_newtype_struct({type_name:?}, &self.0.format({local}time2::{serialize_format}).map_err({local}serde::ser::Error::custom)?)
    }}
}}