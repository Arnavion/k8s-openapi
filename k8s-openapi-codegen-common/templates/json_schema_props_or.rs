enum {type_name} {{
    Schema(Box<{json_schema_props_type_name}>),
    {or_variant_name}({or_variant_type}),
}}

impl crate::DeepMerge for {type_name} {{}}

impl<'de> {local}serde::Deserialize<'de> for {type_name} {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: {local}serde::Deserializer<'de> {{
        struct Visitor;

        impl<'de> {local}serde::de::Visitor<'de> for Visitor {{
            type Value = {type_name};

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str({type_name:?})
            }}

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error> where A: {local}serde::de::MapAccess<'de> {{
                Ok({type_name}::Schema({local}serde::de::Deserialize::deserialize({local}serde::de::value::MapAccessDeserializer::new(map))?))
            }}

{or_visit}        }}

        deserializer.deserialize_any(Visitor)
    }}
}}

impl {local}serde::Serialize for {type_name} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: {local}serde::Serializer {{
        match self {{
            {type_name}::Schema(value) => value.serialize(serializer),
            {type_name}::{or_variant_name}(value) => value.serialize(serializer),
        }}
    }}
}}
