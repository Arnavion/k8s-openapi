struct {type_name}(pub std::string::String);

impl {local}DeepMerge for {type_name} {{
    fn merge_from(&mut self, other: Self) {{
        crate::DeepMerge::merge_from(&mut self.0, other.0);
    }}
}}

impl<'de> {local}serde::Deserialize<'de> for {type_name} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
        struct Visitor;

        impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
            type Value = {type_name};

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                f.write_str("{type_name}")
            }}

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: {local}serde::Deserializer<'de> {{
                deserializer.deserialize_any(self)
            }}

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                Ok({type_name}(std::string::ToString::to_string(&v)))
            }}

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                Ok({type_name}(std::string::ToString::to_string(&v)))
            }}

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                Ok({type_name}(std::string::ToString::to_string(&v)))
            }}

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                self.visit_string(v.into())
            }}

            fn visit_string<E>(self, v: std::string::String) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                Ok({type_name}(v))
            }}
        }}

        deserializer.deserialize_newtype_struct("Quantity", Visitor)
    }}
}}

impl {local}serde::Serialize for {type_name} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: {local}serde::Serializer {{
        serializer.serialize_newtype_struct("Quantity", &self.0)
    }}
}}