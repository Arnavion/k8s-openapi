enum {type_name} {{
    Int(i32),
    String(std::string::String),
}}

impl Default for {type_name} {{
    fn default() -> Self {{
        {type_name}::Int(0)
    }}
}}

impl {local}DeepMerge for {type_name} {{
    fn merge_from(&mut self, other: Self) {{
        *self = other;
    }}
}}

impl<'de> {local}serde::Deserialize<'de> for {type_name} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
        struct Visitor;

        impl {local}serde::de::Visitor<'_> for Visitor {{
            type Value = {type_name};

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
                formatter.write_str({type_name:?})
            }}

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                Ok({type_name}::Int(v))
            }}

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                let v = v.try_into().map_err(|_| {local}serde::de::Error::invalid_value({local}serde::de::Unexpected::Signed(v), &"a 32-bit integer"))?;
                Ok({type_name}::Int(v))
            }}

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                let v = v.try_into().map_err(|_| {local}serde::de::Error::invalid_value({local}serde::de::Unexpected::Unsigned(v), &"a 32-bit integer"))?;
                Ok({type_name}::Int(v))
            }}

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                self.visit_string(v.into())
            }}

            fn visit_string<E>(self, v: std::string::String) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                Ok({type_name}::String(v))
            }}
        }}

        deserializer.deserialize_any(Visitor)
    }}
}}

impl {local}serde::Serialize for {type_name} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: {local}serde::Serializer {{
        match self {{
            {type_name}::Int(i) => i.serialize(serializer),
            {type_name}::String(s) => s.serialize(serializer),
        }}
    }}
}}