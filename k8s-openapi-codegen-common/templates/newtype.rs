struct {type_name}({vis}{inner_type_name});

impl crate::DeepMerge for {type_name} {{
    fn merge_from(&mut self, other: Self) {{
        self.0.merge_from(other.0);
    }}
}}

impl<'de> {local}serde::Deserialize<'de> for {type_name} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
        struct Visitor;

        impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
            type Value = {type_name};

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str({type_name:?})
            }}

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: {local}serde::Deserializer<'de> {{
                Ok({type_name}({local}serde::Deserialize::deserialize(deserializer)?))
            }}
        }}

        deserializer.deserialize_newtype_struct({type_name:?}, Visitor)
    }}
}}

impl {local}serde::Serialize for {type_name} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: {local}serde::Serializer {{
        serializer.serialize_newtype_struct({type_name:?}, {inner_value})
    }}
}}
