enum {type_name} {{
    Int(i32),
    String(String),
}}

impl Default for {type_name} {{
    fn default() -> Self {{
        {type_name}::Int(0)
    }}
}}

impl crate::DeepMerge for {type_name} {{}}

impl<'de> {local}serde::Deserialize<'de> for {type_name} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
        struct Visitor;

        impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
            type Value = {type_name};

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                write!(formatter, {type_name:?})
            }}

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                Ok({type_name}::Int(v))
            }}

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                if v < i64::from(i32::min_value()) || v > i64::from(i32::max_value()) {{
                    return Err({local}serde::de::Error::invalid_value({local}serde::de::Unexpected::Signed(v), &"a 32-bit integer"));
                }}

                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                Ok({type_name}::Int(v as i32))
            }}

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                #[allow(clippy::cast_sign_loss)]
                {{
                    if v > i32::max_value() as u64 {{
                        return Err({local}serde::de::Error::invalid_value({local}serde::de::Unexpected::Unsigned(v), &"a 32-bit integer"));
                    }}
                }}

                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                Ok({type_name}::Int(v as i32))
            }}

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
                self.visit_string(v.to_owned())
            }}

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: {local}serde::de::Error {{
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
