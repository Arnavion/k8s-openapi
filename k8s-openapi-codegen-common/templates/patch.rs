enum {type_name} {{
    Json(std::vec::Vec<{local}serde_json::Value>),
    Merge({local}serde_json::Value),
    StrategicMerge({local}serde_json::Value),
}}

impl {local}serde::Serialize for {type_name} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: {local}serde::Serializer {{
        match self {{
            {type_name}::Json(patch) => serializer.serialize_newtype_struct({type_name:?}, patch),
            {type_name}::Merge(patch) |
            {type_name}::StrategicMerge(patch) => serializer.serialize_newtype_struct({type_name:?}, patch),
        }}
    }}
}}