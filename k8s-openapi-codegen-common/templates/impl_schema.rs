
{cfg}impl{type_generics_impl} {local}schemars::JsonSchema for {type_name}{type_generics_type}{type_generics_where} {{
    fn schema_name() -> String {{
        {definition_path:?}.to_owned()
    }}

    fn json_schema(__gen: &mut {local}schemars::gen::SchemaGenerator) -> {local}schemars::schema::Schema {{
{schema}    }}
}}
