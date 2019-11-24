enum {type_name} {{
    Json(Vec<serde_json::Value>),
    Merge(serde_json::Value),
    StrategicMerge(serde_json::Value),
}}

impl serde::Serialize for {type_name} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {{
        match self {{
            {type_name}::Json(patch) => serializer.serialize_newtype_struct({type_name:?}, patch),
            {type_name}::Merge(patch) |
            {type_name}::StrategicMerge(patch) => serializer.serialize_newtype_struct({type_name:?}, patch),
        }}
    }}
}}